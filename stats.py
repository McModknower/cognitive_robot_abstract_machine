import pstats
import sys
if len(sys.argv) != 3:
    print(f'usage: {sys.argv[0]} inputname sortkey')
    exit(1)
stats = pstats.Stats(sys.argv[1])
key = sys.argv[2]
if key == 't':
    key = 'time'
elif key == 'c':
    key = 'cumulative'
stats.sort_stats(key).print_stats()
