from sqlalchemy import create_engine
import pandas as pd
from dotenv import load_dotenv
import os

load_dotenv()

db_url = os.getenv("DATABASE_URL_SQLALCHEMY")
if db_url is None:
    raise ValueError("DATABASE_URL_SQLALCHEMY not found in .env file")

engine = create_engine(db_url)

tracking_info_df = pd.read_sql("SELECT * FROM tracking_info", engine)
timestamps_df = pd.read_sql("SELECT * FROM timestamps", engine)

print(tracking_info_df.head())
print(timestamps_df.head())