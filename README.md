# supersync (experimental)

Always had a problem with the smb protocol not being fast/efficient enough, even over gigabit networks to realtime update shared files, when you don't access to a NAS.

## Concept

Watch changes, get a diff, sync both client directories over a socket in realtime.
