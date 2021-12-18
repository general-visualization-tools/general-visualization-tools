import concurrent.futures
import subprocess
import json
import logging
import re
import datetime
import enum


# logger の設定
logging.basicConfig(level=logging.DEBUG, format="(%(levelname)s: %(name)s)  %(message)s")
logger = logging.getLogger(__name__)


class ExecutionResultState(enum.Enum):
    SolverFailed = 'solver failed'
    ScoringFailed = 'scoring failed'
    Succeeded = 'succeeded'

    def __str__(self):
        return self.value


def run(setting_filepath: str, *, out_filepath='./output.json'):
    # 設定ファイルを読み込む
    with open(setting_filepath) as f:
        setting_dict = json.load(f)

    # 設定ファイルから読み込んだデータを格納
    in_and_out_filepaths: list[dict[str, str]] = setting_dict["files"]
    solve_command: str = _concat_commands(setting_dict['solver'])
    score_command: str = _concat_commands(setting_dict['scoring'])
    max_workers:   int = setting_dict['concurrency']

    scores: list[float] = []
    execution_results: list[dict] = []
    is_ac = True
    number_of_files = len(in_and_out_filepaths)
    # ソルバーと採点を並列実行
    with concurrent.futures.ProcessPoolExecutor(max_workers=max_workers) as executor:
        for result in executor.map(
                _single_solve_and_score,
                in_and_out_filepaths,
                [solve_command for _ in range(number_of_files)],
                [score_command for _ in range(number_of_files)]
        ):
            # ひとつでも失敗があればWAとする
            if result.state != ExecutionResultState.Succeeded:
                is_ac = False
            # スコアを配列に格納(後で平均等を計算するため)
            scores.append(result.score)

            # 実行結果を格納
            # 失敗時のデータも含める
            execution_results.append(result.to_dict())

            logger.info(f'{result.in_filepath} {result.state}')

    # 出力処理
    scores.sort()
    n = len(scores)
    # 後でjsonに変換する辞書を作成
    output_dict = {
        'state': 'AC' if is_ac is True else 'WA',
        'date': datetime.datetime.now().strftime('%Y:%m:%d %H:%M:%S'),
        'score': {
            'ave': sum(scores) / n if n != 0 else 0,
            'sum': sum(scores),
            'min': scores[0] if n != 0 else 0,
            'max': scores[-1] if n != 0 else 0
        },
        'detailed': execution_results
    }
    # jsonを指定されたパスに出力する
    with open(out_filepath, 'w') as f:
        json.dump(output_dict, f, indent=2, ensure_ascii=False)


class ExecutionResult:
    # ソルバーと採点の実行結果を表すクラス
    in_filepath: str
    out_filepath: str
    state: ExecutionResultState
    error_code: str
    score: float
    exec_ms_time: int
    memory_used: int

    def __init__(
        self,
        state: ExecutionResultState,
        in_filepath: str,
        out_filepath: str,
        *,
        error_code: str = '',
        score: float = 0.0,
        exec_ms_time: int = 0,
        memory_used: int = 0
    ):
        self.state = state
        self.in_filepath = in_filepath
        self.out_filepath = out_filepath
        self.error_code = error_code
        self.score = score
        self.exec_ms_time = exec_ms_time
        self.memory_used = memory_used

    def to_dict(self):
        return {
            'state': str(self.state),
            'inFilepath': self.in_filepath,
            'outFilepath': self.out_filepath,
            'errorCode': self.error_code,
            'score': self.score,
            'execMsTime': self.exec_ms_time,
            'memoryUsed': self.memory_used
        }


def _single_solve_and_score(in_and_out_filepath: dict[str, str], solve_command: str, score_command: str) -> ExecutionResult:
    in_filepath = in_and_out_filepath["in"]
    out_filepath = in_and_out_filepath["out"]

    # ソルバーを実行
    finished_solver = subprocess.run(solve_command, shell=True, text=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE, env={'IN_PATH': in_filepath, 'OUT_PATH': out_filepath})
    # ソルバーの実行の失敗時
    if finished_solver.returncode != 0:
        return ExecutionResult(ExecutionResultState.SolverFailed, in_filepath, out_filepath, error_code=finished_solver.stderr)

    # 採点を実行
    finished_score = subprocess.run(score_command, shell=True, text=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE, env={'IN_PATH': in_filepath, 'OUT_PATH': out_filepath})
    # 採点の実行の失敗時
    if finished_score.returncode != 0:
        return ExecutionResult(ExecutionResultState.ScoringFailed, in_filepath, out_filepath, error_code=finished_score.stderr)
    # ソルバーと採点の実行の成功時
    else:
        # 採点結果を取り出す
        score = _extract_scoring_output(finished_score.stdout)
        return ExecutionResult(ExecutionResultState.Succeeded, in_filepath, out_filepath, score=score)


def _concat_commands(commands: list[str]):
    # 設定ファイルから、ソルバーや採点のプログラムの実行コマンドが配列として渡される。
    # それらを && で結合して、単一の文字列にする
    return ' && '.join(commands)


def _extract_scoring_output(s: str):
    # 外部で用意される採点プログラムは、点数以外の出力を含む場合がある
    # そのためそれらから、点数の部分のみを取り出しfloat型として返す
    extract_str = re.search('(?<==)\s*\d*$', s)
    if extract_str is None:
        # TODO: 将来的にはエラーを投げる
        logger.debug('採点プログラムの出力から上手く点数を抽出できませんでした。\n')
        score = 0.0
    else:
        score = float(extract_str.group(0))
    return score


if __name__ == '__main__':
    # マジック変数
    INPUT_JSON = 'setting.json'

    run(INPUT_JSON)
