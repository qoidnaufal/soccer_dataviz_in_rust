import pandas as pd

df = pd.read_csv("dataset/liga1.csv")
df_cka = df[["Team", "Opponent", "Match", "Shot on - Corner Kick"]]
df_cka = df_cka.groupby(["Team"]).sum()
print(df_cka)
