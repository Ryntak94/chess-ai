f = open("twicPGN/twic920.pgn", "r")
lines = f.readlines()
for line in lines:
    if line[0] != "[" and line[0] != "\n":
        print(line)
