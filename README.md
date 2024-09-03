# PoC: Grafana API automation

## Resources:

- api/users
- api/teams
- api/folders
- api/folders/:folder_uid/permissions

## Use cases:

- Create admin and viewer team
- Create folder for teams
- Set folder permissions
- Add annotations to panels
- Add dashboards to folders

## Requests:

- CreateTeamRequest
- CreateFolderRequest
- UpdateFolderPermissionRequest
- DeleteTeamRequest
- DeleteTeamsWithZeroMembers

## Responses:

- GetTeamResponse
- CreateTeamResponse
- SearchTeamsResponse
- UpdateFolderPermissionResponse
- CreateFolderResponse
- QueryFolderResponse
- GetFolderResponse

## Use case 1: Teams (admin/viewer), Folders, Permissions

```shell
# add a grafana team with admin rights with the name "Team1" and a viewer team with suffix "-Viewer"
gfi.exe add team -n Team17

# add two teams and also add a grafana dashboard directory for the two teams with permissions set.
gfi.exe add team -n Team17 -d

# get a team by its id
gfi.exe get team -i 17

# query teams by name "Team*"
gfi.exe get team -q Team

# delete a team by its id
gfi.exe del team -i 17

# delete all teams with zero members (delete confirmation required for each team with zero members)
gfi.exe del team -z

# delete all teams with zero members (delete confirmation prematurely given for all teams with zero members)
gfi.exe del team -z -y

# add folder without team
gfi.exe add folder -t Folder17

# get folder by its uid
gfi.exe get folder -u fdu0hhbnheoe8a

# get all folders
gfi.exe get folder

# get folders with limit and page 
gfi.exe get folder -l 3 -p 1

# override folder permissions for only one team with admin permissions in this case
gfi.exe add permission -f fdu0hhbnheoe8a -t 17 -p 4 
```

## Use case 2: Dashboards, Annotations, Folders

```shell
# add a dashboard with the name Dashboard17, a tag with text templated, browser timezone, schema_version 16, 
# refresh rate in seconds, a simple message and a newly created folder with the title Folder17 where the dashboard moves into
gfi.exe add dashboard -n Dashboard17 -t templated -z browser -s 16 -r 25 -m "add dashboard into a newly created folder" -c Folder17

# add a dashboard into an existing folder by specifying the folder_uid
gfi.exe add dashboard -n Dashboard17 -t templated -z browser -s 16 -r 25 -m "add dashboard into an existing folder" -f edwro045bsg74b

# get a dashboard with its meta info by its uid
gfi.exe get dashboard -u cdwrrb1xgx5vkb
```