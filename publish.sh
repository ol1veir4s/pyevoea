#!/bin/bash

hugo 
git add .
git commit -m "publish.sh: updates"
git push && git pull