# Progrs

Small CLI-based todo app for managing your work progress.

## Goal
1. Users are able to add their todo work (status: ongoing).
2. Users are able to see all their todo works (all status).
3. Users are able to edit their todo works and the status.
4. Users are able to delete their todo works (not recommended)
5. Users are able to see the status of each todo works.

## Schema
```
    id: u32,
    todo_name: String,
    start_date: date,
    end_date: date,
    notes: String,
    status: Ongoing | Finished | Ignored
```
