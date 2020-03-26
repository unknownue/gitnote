
# date "+DATE: %Y-%m-%d%nTIME: %H:%M:%S %Z"
set --local TIMESTAMP (date "+%m/%d %Z %Y")

git add --all
git commit -m "AutoCommit on $TIMESTAMP."
git push
