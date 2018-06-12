/*

In std::result::Result:

  enum Result {
    Ok(T),
    Err(E),
  }

Note: Result is part of the Prelude, so you can use Ok() and Err() directly

If you run a function that might fail, you'll usually get a Result type back.
You can then run a match expression to handle the Result appropriately:

  match function_that_might_fail() {
    Ok(value) => handle success,
    Err(value) => handle error,
  }

If you have multiple operations, that might fail, and you wrap them into a function,
that returns a Result type, you can use the ? Operator.

Example Function:

  fn read_file() -> Result<String, io::Error> {
    1. try to open a file handle
    2. try to read from the file handle
  }

The ? Operator will yield the value contained in the Ok(T) on success [for 'Ok("quack")' it will yield '"quack"']
and it will return the Err(E) if the function fails [for 'Err(Error)' it will run 'return Err(Error)', exiting from read_file()]

The ? Operator is useful if you have more than one Try-operation and want to chain them, because you don't need to destructure the Ok(T) upon success.
Of course it's also a shorthand for the following match operation:

  match function_that_might_fail() {
    Ok(value) => value,
    Err(value) => return Err(value),
  }

which will return from a function with Err(value) if function_that_might_fail() fails, and which will yield value to any receiver upon success.
Such a match will usually be binding the value to a variable, so it can be used afterwards.

See this:

  fn test() -> Result<T,E> {
    let a = function_that_might_fail();

    let a = match a {
      Ok(value) => value,			// bind value to a (match yields value to =, which binds it to a)
      Err(error) => return Err(error),		// return with Err(value) from test(), in which case no binding to a is made (so no problems with the error's type there)
    };

    Ok(a)					// to satisfy the test()'s return type, package a up in an Ok ... Ok(a) == Ok(value)
  }

The previous is functionally the same as this:

  fn test() -> Result<T,E> {
    let a = function_that_might_fail();

    let a = a?;
    
    Ok(a)
  }

Since test() returns a Result, you have to manually rewrap a in an Ok upon success, because ? destructures an Ok.

? can only be used in Functions that return a Result, cause if it gets an Err(E) it will run 'return Err(E)' and return from the function [before that function is completed]
For the types to match up, the function then obviously needs to return a Result type.

In a binding operation you don't get any issues this way, because ? will either return a valid type (like String) or it will return from the function before the binding is completed.
? doesn't yield to the binding operation, but to the function caller, if it encounters an Err type.

*/

/*

'It is a unary suffix operator which can be placed on an expression to unwrap the value on the left hand sife of '?' while propagating any error through an early return.'

? takes a Result type and either unwraps the Ok to yield the value inside it, or propagate the Err to the function caller through an early return. 

? does the same as the match-expression in this test() function:

fn test() -> Result<T,e> {

  let f = match a {		// if a value is yielded from the match expression, bind it to f
    Ok(value) => value,		// unwrap Ok(value) and yield value from the match-expression
    Err(e) => return Err(e),	// propagate Err(e) to the function caller through an early return from test()
  }

}

*/
