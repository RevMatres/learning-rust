/*


A public item can be accessed by any higher scope, than its own, as well as its own scope.
A private item can only be accessed by its own scope (it's local to that scope) and children of that scope.

If a private item contains a public item, the scope containing the private item can access the public item,
but no higher scope than that can access the public item.

root {

  // can access main
  // thereby can access main::child

  // can't access main::parent cause it's private/local to main

  main {
  
    // can access parent
    // thereby can access parent::child1
  
    // can't access parent::child2 cause it's private/local to parent
  
    parent {			[private]
      pub child1 {}
      child2 {} 		[private]
    }

    pub child {}
  
  }


  // can access child
  // thereby can access child::child

  pub child {
    pub child {}
  }

}

Private things are local to their scope.

Anything public can be accessed by any higher scope, provided all scopes separating them can be accessed as well.
If the separating scopes are public, the public thing can be accessed.
If there is only one private scope separating them, the private scope is local to the accessing scope and the public thing can be accessed.
If there are more private scopes separating them, the public thing can't be accessed anymore.

*/
