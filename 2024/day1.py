import pandas as pd
import os

# Read input data
script_dir = os.path.dirname(__file__)
input_file_path = os.path.join(script_dir, 'day1_input.txt')

with open(input_file_path, 'r') as file:
    input_data = file.read()

lines = input_data.strip().split('\n')

data = [line.split() for line in lines]

# Create a DataFrame
df = pd.DataFrame(data)

# Order each column from lowest to highest
df_ordered = df.apply(lambda x: sorted(x))

# Compare values in each column to create third column of differences
df_ordered['differences'] = df_ordered.apply(lambda x: int(x[1]) - int(x[0]), axis=1)

# Change all difference values to be absolute
df_ordered['differences'] = df_ordered['differences'].abs()

# Part 1 ANSWER
print(df_ordered['differences'].sum())

# -----------------------------------------------------------------------------

# Create a new data frame from the unique values in column 1
df_part_two = pd.DataFrame(df_ordered[0].unique())

# For every value in the new column, count the number of times it appears in the second column of the original dataframe
df_part_two['count'] = df_part_two[0].apply(lambda x: (df_ordered[1] == x).sum())

# Similarity scoring
df_part_two['score'] = df_part_two[0].astype(int) * df_part_two['count']

# Debugging
# df_part_two.to_csv('day1_testoutput.csv', index=False)

# Part 2 ANSWER
print(df_part_two['score'].sum())
