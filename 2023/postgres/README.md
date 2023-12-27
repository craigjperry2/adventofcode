## Developer Setup Notes

### Running Postgres

I'm running postgres in rootless podman on MacOS (arm64) & Fedora (x64). I've created some aliases to make managing
these faster:

    alias ppp="podman pull docker.io/library/postgres"

    # create a new instance, you can just podman start/stop this if you don't need to blow it away each time
    alias prp="podman run --name postgres -e POSTGRES_PASSWORD=notsecure -d -p 5432:5432 postgres"

    # Copy psqlrc customisations into the container when running psql inside the container (e.g. for operations benefiting from faster unix domain socket connectivity vs TCP - bulk copies etc.)
    alias pcp="podman cp $HOME/.psqlrc postgres:/root/.psqlrc"

    # Run psql inside the container via unix domain socket
    alias ppsql="pcp ; podman exec -it postgres psql -U postgres"

    # Run client on dev machine (Mac or Linux) and connect into container over the exposed 5432 TCP port
    alias psql="env PGPASSWORD=notsecure ${PSQL_PATH}psql -U postgres -h localhost"

    alias pprm="podman stop postgres ; podman rm postgres"

You can see the context for these at: https://github.com/craigjperry2/dotfiles/blob/main/dotfiles/zshrc

### Database Client

I almost always use the `psql` cli (with a lightly customised `.psqlrc`) but for this i'm using JetBrains DataGrip IDE -
which i think is just the database parts of intellij. I like the intellij DB client, they recently added graph
visualisation there and i find that handy!

* I had to add the Markdown plugin - that wasn't installed automatically like IDEA.
* I had to add the docker plugin to get in-IDE views of the podman container. I don't find the intellij docker plugin
  all that useful but sometimes the show files view is handy.
* I had to add the terminal plugin - that seems extreme not to include that by default

End of experiment... i went back to vanilla intellij idea :-)
