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


def run(settin_filepath: str, *, out_filepath='./output.json'):
    # 設定ファイルを読み込む
    with open(settin_filepath) as f:
        setting_dict = json.load(f)

    # 設定ファイルから読み込んだデータを格納
    in_and_out_filepaths: list[dict[str, str]] = setting_dict["files"]
    solve_command: str = _concat_commands(setting_dict['solver'])
    score_command: str = _concat_commands(setting_dict['scoring'])
    max_workers:   int = setting_dict['concurrency']

    scores: list[float] = []
    executionResults: list[dict] = []
    is_AC = True
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
            if result.state != 'succeeded':
                is_AC = False
            else:
                # スコアを配列に格納(後で平均等を計算するため)
                # 失敗時のスコアは含めない
                scores.append(result.score)
            # 実行結果を格納
            # 失敗時のデータも含める
            executionResults.append(result.to_dict())

            logger.info(f'{result.in_filepath} {result.state}')

    # 出力処理
    scores.sort()
    n = len(scores)
    # 後でjsonに変換する辞書を作成
    output_dict = {
        'state': 'AC' if is_AC is True else 'WA',
        'date': datetime.datetime.now().strftime('%Y:%m:%d %H:%M:%S'),
        'score': {
            'ave': sum(scores) / n if n != 0 else 0,
            'sum': sum(scores),
            'min': scores[0] if n != 0 else 0,
            'max': scores[-1] if n != 0 else 0
        },
        'detailed': executionResults
    }
    # jsonを指定されたパスに出力する
    with open(out_filepath, 'w') as f:
        json.dump(output_dict, f, indent=2, ensure_ascii=False)


class ExecutionResult:
    # ソルバーと採点の実行結果を表すクラス
    in_filepath: str
    out_filepath: str
    state: ExecutionResultState
    errorcode: str
    score: float
    execMsTime: int
    memoryUsed: int

    def __init__(
        self,
        state: ExecutionResultState,
        in_filepath: str,
        out_filepath: str,
        *,
        errorcode: str = '',
        score: float = 0.0,
        execMsTime: int = 0,
        memoryUsed: int = 0
    ):
        self.state = state
        self.in_filepath = in_filepath
        self.out_filepath = out_filepath
        self.errorcode = errorcode
        self.score = score
        self.execMsTime = execMsTime
        self.memoryUsed = memoryUsed

    def to_dict(self):
        return {
            'state': str(self.state),
            'in_filepath': self.in_filepath,
            'out_filepath': self.out_filepath,
            'errorcode': self.errorcode,
            'score': self.score,
            'execMsTime': self.execMsTime,
            'memoryUsed': self.memoryUsed
        }


def _single_solve_and_score(in_and_out_filepath: dict[str, str], solve_command: str, score_command: str) -> ExecutionResult:
    in_filepath = in_and_out_filepath["in"]
    out_filepath = in_and_out_filepath["out"]

    try:
        # ソルバーを実行
        finished_solver = subprocess.run(solve_command, shell=True, text=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE, env={'IN_PATH': in_filepath, 'OUT_PATH': out_filepath})
        # ソルバーが正しく終了をしていなければこの関数から例外が投げられる
        finished_solver.check_returncode()
    # ソルバーの実行の失敗時
    except subprocess.CalledProcessError:
        result = ExecutionResult(ExecutionResultState.SolverFailed, in_filepath, out_filepath, errorcode=finished_solver.stderr)

    try:
        # 採点
        finished_score = subprocess.run(score_command, shell=True, text=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE, env={'IN_PATH': in_filepath, 'OUT_PATH': out_filepath})
        # 実行ファイルが正しく終了をしていなければこの関数から例外が投げられる
        finished_score.check_returncode()
    # 採点の失敗時
    except subprocess.CalledProcessError:
        result = ExecutionResult(ExecutionResultState.ScoringFailed, in_filepath, out_filepath, errorcode=finished_score.stderr)
    else:
        # 採点結果を取り出す
        score = _extract_scoring_output(finished_score.stdout)
        result = ExecutionResult(ExecutionResultState.Succeeded, in_filepath, out_filepath, score=score)

    return result


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
