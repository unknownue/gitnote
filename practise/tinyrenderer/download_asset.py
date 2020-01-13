#!/usr/bin/env python3

# This script is modified from https://github.com/SaschaWillems/Vulkan/blob/master/download_assets.py

import sys
import os
from urllib.request import urlretrieve
from zipfile import ZipFile

boggie = [
    "body.obj",
    "body_diffuse.tga",
    "body_nm_tangent.tga",
    "body_spec.tga",
    "eyes.obj",
    "eyes_diffuse.tga",
    "eyes_nm_tangent.tga",
    "eyes_spec.tga",
    "head.obj",
    "head_diffuse.tga",
    "head_nm_tangent.tga",
    "head_spec.tga",
    "readme.txt",
]

diablo3_pose = [
    "diablo3_pose.obj",
    "diablo3_pose_diffuse.tga",
    "diablo3_pose_glow.tga",
    "diablo3_pose_nm.tga",
    "diablo3_pose_nm_tangent.tga",
    "diablo3_pose_spec.tga",
    "readme.txt",
]

african_head = [
    "african_head.obj",
    "african_head_diffuse.tga",
    "african_head_eye_inner.obj",
    "african_head_eye_inner_diffuse.tga",
    "african_head_eye_inner_nm.tga",
    "african_head_eye_inner_nm_tangent.tga",
    "african_head_eye_inner_spec.tga",
    "african_head_eye_outer.obj",
    "african_head_eye_outer_diffuse.tga",
    "african_head_eye_outer_gloss.tga",
    "african_head_eye_outer_nm.tga",
    "african_head_eye_outer_nm_tangent.tga",
    "african_head_eye_outer_spec.tga",
    "african_head_nm.tga",
    "african_head_nm_tangent.tga",
    "african_head_spec.tga",
    "african_head_SSS.jpg",
    "readme.txt",
]

remaining_assets = [
    "floor.obj",
    "floor_diffuse.tga",
    "floor_nm_tangent.tga",
    "grid.tga"
]

BASE_URL = "https://github.com/ssloy/tinyrenderer/raw/master/obj/"

def reporthook(blocknum, blocksize, totalsize):
    bytesread = blocknum * blocksize
    if totalsize > 0:
        percent = bytesread * 1e2 / totalsize
        s = "\r%5.1f%% (%*d / %d bytes)" % (percent, len(str(totalsize)), bytesread, totalsize)
        sys.stderr.write(s)
        if bytesread >= totalsize:
            sys.stderr.write("\n")
    else:
        sys.stderr.write("read %d\n" % (bytesread,))

print("Downloading african head...")
destination_url = "./assets/african_head"
if not os.path.exists(destination_url):
    os.makedirs(destination_url)
for asset in african_head:
    urlretrieve(f"{BASE_URL}/african_head/{asset}", f"{destination_url}/{asset}", reporthook)

print("Downloading boggie...")
destination_url = "./assets/boggie"
if not os.path.exists(destination_url):
    os.makedirs(destination_url)
for asset in boggie:
    urlretrieve(f"{BASE_URL}/boggie/{asset}", f"{destination_url}/{asset}", reporthook)

print("Downloading diablo3 pose...")
destination_url = "./assets/diablo3_pose"
if not os.path.exists(destination_url):
    os.makedirs(destination_url)
for asset in diablo3_pose:
    urlretrieve(f"{BASE_URL}/diablo3_pose/{asset}", f"{destination_url}/{asset}", reporthook)

print("Downloading remaining assets...")
for asset in remaining_assets:
    download_url = f"{BASE_URL}/{asset}"
    destination_url = f"./assets/{asset}"
    urlretrieve(download_url, destination_url, reporthook)

print("Download finished")
