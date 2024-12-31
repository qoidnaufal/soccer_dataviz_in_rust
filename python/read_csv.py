import pandas as pd

df = pd.read_csv("dataset/liga1.csv")
df_ck = df[["Name", "Team", "Opponent", "Shot on - Corner Kick"]]
print(df_ck)
