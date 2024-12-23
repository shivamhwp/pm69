# pm69 -- a package manager.

consider this (for me)

1. versions for pkgs or repos. support for "latest". we store the latest version in local spec, not the "latest" keyword.

2. lockfile: helps in installing pkgs using your cli on other places than your project.

### preassumptions while building pm69

1. if there is a .pm69 folder in current dir,use project scope.

2. whenever run pm69 install owner/name

   > goto `https://github.com/OWNER/NAME`, if not found then
   > fetch it from pkg registry.
   > (you can also add the functionality to directly download from diff urls or local paths)

3. the only source of truth, and it's only the contents of `.pm69` folder.

4. we maybe leaving pkg dependencies. like let's say two packages use same dependecy, but they are using diff versions, which can cause conflicts.

5. ~
