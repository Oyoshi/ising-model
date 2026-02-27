import pandas as pd
import matplotlib.pyplot as plt

df = pd.read_csv("magnetization_data_classic.csv")

plt.figure(figsize=(12, 7))

plt.fill_between(
    df["temp"],
    df["mag"] - df["std_dev"],
    df["mag"] + df["std_dev"],
    color="mediumturquoise",
    alpha=0.2,
    label="Standard Deviation",
)

plt.errorbar(
    df["temp"],
    df["mag"],
    yerr=df["std_dev"],
    fmt="o-",
    markersize=3,
    linewidth=1,
    color="mediumturquoise",
    ecolor="paleturquoise",
    capsize=2,
    label="Mean Magnetization",
)

plt.axvline(
    x=2.269,
    color="mediumvioletred",
    linestyle="--",
    alpha=0.7,
    label="Theoretical T_c",
)

plt.title(
    "Ising Model (classic): Magnetization with Critical Fluctuations", fontsize=14
)
plt.xlabel("T", fontsize=12)
plt.ylabel("<M>", fontsize=12, rotation=0, labelpad=20)
plt.grid(True, which="both", linestyle="--", alpha=0.5)
plt.legend()

plt.savefig("ising_classic_plot.png", dpi=300)
plt.show()
