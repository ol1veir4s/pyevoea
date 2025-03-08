#!/bin/bash

hugo 
git add .
git commit -m "deploy.sh: updates"
git push && git pull