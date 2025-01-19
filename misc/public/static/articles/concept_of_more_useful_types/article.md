# Concept of more useful types

You've probably written something takes filename as an argument
```python
def process_filename(filename: str) -> None: ...
```

And it was fine until you have not had a filename, but already retrieved bytes
```python
value: bytes
process_filename(value) # incompatible
```

You could write bytes to disk and get a filename and make it work, albeit worsening performance among other things

*Bytes → Filename → Process Filename*

```python
value: bytes
filename = write_to_disk(value)
process_filename(filename)
```

But data is more often received as bytes, not filename
So identifying a more useful type would have saved you jumping through hoops
> in this case bytes is more useful than filename

*Bytes → Process Bytes*

```python
# def process_filename(filename: str) -> None: ...
def process_bytes(bytes: bytes) -> None: ...

value: bytes
process_bytes(value)
```

*process_bytes* would still be useful if you had filename, or retrived data from other sources

*Filename → Bytes → Process Bytes*

```python
filename: str
with open(filename, "rb") as f:
    bytes_from_disk = f.read()
process_bytes(bytes_from_disk)

process_bytes(bytes_from_network)
process_bytes(bytes_from_channel)
```

## Conclusion
Identifying more useful types would save you from later refactoring, positively impact performance by cutting inderection and aid in writing reusable code
