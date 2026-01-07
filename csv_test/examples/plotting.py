import japanize_matplotlib  # noqa
import pandas as pd
import matplotlib.pyplot as plt

# CSVファイルの読み込み
df = pd.read_csv("output.csv")

# グラフの描画
plt.figure(figsize=(10, 6))
plt.plot(df["t"], df["x"], label="位置 x")
plt.plot(df["t"], df["v"], label="速度 v")
plt.xlabel("時間 t")
plt.ylabel("値")
plt.legend()
plt.grid(True)
plt.savefig("plot_py.png")
plt.show()
