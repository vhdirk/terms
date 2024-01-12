# Resets Terms settings to default for testing purposes
gsettings list-schemas | grep Terms | xargs -n 1 gsettings reset-recursively
dconf list /io/github/vhdirk/Terms/ | xargs -I {} dconf reset -f "/io/github/vhdirk/Terms/"{}
