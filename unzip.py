import zipfile
import os
iterate = True
num = 920
if not os.path.isdir("twicPGN"):
    os.mkdir("twicPGN")

while iterate:
    try:
        with zipfile.ZipFile('twicZips/twic' + str(num) + '.zip', 'r') as zip_ref:
            zip_ref.extractall('twicPGN')
        num+=1
    except Exception as e:
        print(e)
        iterate = False
