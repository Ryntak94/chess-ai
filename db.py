import urllib.request
import os
import zipfile
if not os.path.isdir("twicZips"):
    os.mkdir("twicZips")
iterate = True
num = 920
while iterate:
    print("while block")
    try:
        print("try")
        f = urllib.request.urlretrieve("https://theweekinchess.com/zips/twic"+ str(num) + "g.zip", "twicZips/twic" + str(num) + ".zip")
        num+=1
        print(f)
    except Exception as e:
        print(e)
        iterate = False

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
