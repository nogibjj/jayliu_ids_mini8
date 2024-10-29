import pandas as pd


def process_birth_data(file_path):
    data = pd.read_csv(file_path)

    data_filtered = data[data["year"] >= 2000]

    summary_by_month = data_filtered.groupby("month")["births"].sum().reset_index()
    summary_by_day = data_filtered.groupby("day_of_week")["births"].sum().reset_index()

    summary_by_month.to_csv("data/birth_summary_by_month.csv", index=False)
    summary_by_day.to_csv("data/birth_summary_by_day.csv", index=False)

    return summary_by_month, summary_by_day


if __name__ == "__main__":

    month_summary, day_summary = process_birth_data("data/US_birth.csv")
    print("Monthly Summary:\n", month_summary)
    print("Day of Week Summary:\n", day_summary)
