# preprocess-dataset
This executable is run in the directory that contains `combined_data_*.txt`,
formats them into a matrix with columns `(user, movie, rating, year, day_of_year, day_of_week)`,
then writes the output to `output.npy`,
which can be loaded efficiently using the `numpy.load` method.
