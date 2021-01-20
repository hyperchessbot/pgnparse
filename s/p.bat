python s/gen.py


git config --global user.email "hyperchessbot@gmail.com"
git config --global user.name "hyperchessbot"

git checkout -b master

git add . -A

git commit -m "$*"

git push github master

git push gitlab master

