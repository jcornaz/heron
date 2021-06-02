# Interacting with physics

Heron aims to make interaction with physics as straight forward as possible.

In general all you have to do is read/mutate for the relevant components. (`Transform`, `Velocity`, `Acceleration`, etc.)

That being said. To avoid frame delay, consider the following advices:

1. Mutate the components during the `CoreStage::Update` stage. (Or before)
2. React on the physics results in the `CoreStage::PostUpdate` stage, and use `PhysicsSystem` labels to declare after
   which system it should run.
