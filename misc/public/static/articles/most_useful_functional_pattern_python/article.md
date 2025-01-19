### Problem

**Python without external libraries lacks disjoined unions**

You may model this
```rust
enum Currency {
    Euro,
    Dollar,
    Peanuts,
}
```

Like so
```python
from enum import Enum

class Currency(Enum):
    EURO = "Euro"
    DOLLAR = "Dollar"
    PEANUTS = "Peanuts"
```

But what about
```rust
enum Action {
    Start,
    Move { x: i32, y: i32 },
}
```

### Functional solution

There comes pydantic and definition of Action
```python
import pydantic
from typing import Annotated
from typing import Literal

class Start(pydantic.BaseModel):
    choice: Literal["start"] = "start"

class Move(pydantic.BaseModel):
    choice: Literal["move"] = "move"
    x: int
    y: int

Action = Annotated[Start | Move, pydantic.Field(discriminator="choice")]
```

Variants defined as variables
```python
action: Action = Start()
action: Action = Move(x=1, y=2)
```

Conditioning based on variable class
```python
if isinstance(action, Start):
    print(f"started")

elif isinstance(action, Move):
    print(f"moved {action.x=!r}")
    print(f"moved {action.y=!r}")
```

Or conditioning based on *choice* attribute 
> useful due to functional limitations, for example in Jinja/Django templates
> or when bringing class references is burdensome
```python
if isinstance(action, Start):
    print(f"started")

elif isinstance(action, Move):
    print(f"moved {action.x=!r}")
    print(f"moved {action.y=!r}")
```

Parsing json using Action results in dispatch to the proper variants
```python
start_json = "{\"choice\": \"start\"}"
assert isinstance(pydantic.RootModel[Action].model_validate_json(start_json).root, Start)
move_json = "{\"choice\": \"move\", \"x\": 1, \"y\": 2}"
assert isinstance(pydantic.RootModel[Action].model_validate_json(move_json).root, Move)
```

#### *Bonus*, with generics:
```python

from typing import Generic, TypeVar, Union, Literal

T = TypeVar("T")
E = TypeVar("E")


# I recommend defining a base model with arbitrary_types_allowed
# and using this for everything because default is too restrictive
class BaseModel(pydantic.BaseModel):
    model_config = pydantic.ConfigDict(arbitrary_types_allowed=True)

# For models that need serialization/deserialization opt in for SerializableBaseModel
class SerializableBaseModel(pydantic.BaseModel):
    model_config = pydantic.ConfigDict(arbitrary_types_allowed=False)


class Ok(BaseModel, Generic[T]):
    choice: Literal["Ok"] = "Ok"
    value: T

class Err(BaseModel, Generic[E]):
    choice: Literal["Err"] = "Err"
    value: E

class Result(BaseModel, Generic[T, E]):
    value: Annotated[Ok[T] | Err[E], pydantic.Field(discriminator="choice")]


MyResult = Result[int, Exception]

ok_value: MyResult = Ok(value=1)
err_value: MyResult = Err(value=ZeroDivisionError())
```

### Conclusion
This is a lifesafer for functional python programming