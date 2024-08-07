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

## Usage:

````shell
# add a grafana team with admin rights with the name "Team1" and a viewer team with suffix "-Viewer"
fi.exe add team -n Team17

# add two teams and also add a grafana dashboard directory for the two teams with permissions set.
fi.exe add team -n Team17 -d

# get a team by its id
fi.exe get team -i 17

# query teams by name "Team*"
fi.exe get team -q Team

# delete a team by its id
fi.exe del team -i 17

# delete all teams with zero members (confirmation required)
fi.exe del team -z

# add folder without team
fi.exe add folder -t Folder17

# get folder by its uid
fi.exe get folder -u fdu0hhbnheoe8a

# get all folders
fi.exe get folder

# get folders with limit and page 
fi.exe get folder -l 3 -p 1

# override folder permissions for one admin team
fi.exe add permission -f fdu0hhbnheoe8a -t 17 -p 4 
````