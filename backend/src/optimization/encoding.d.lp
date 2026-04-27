%*! slot_chosen(SlotId, ItemId)

An item that is chosen for a slot.

This item may be a fixed or a crafted item.

Args:
- SlotId: The slot for which the item is chosen.
- ItemId: The unique identifier of the item that is chosen for the slot.

*%

%*! effective_bonus(ItemId, Stat, Value)

The effective bonus of an item for a specific stat.

This is used to combine item bonuses with gem bonuses and to map item
bonuses to the respective stat of the class. For example, `acuity` is a stat
that may or may not increase the value of some specific stat of a class.

Args:
- ItemId: The unique identifier of the item.
- Stat: The stat for which the effective bonus is calculated.
- Value: The value of the effective bonus for the stat.
*%

%*! slotted_gem(ItemId, GemId)

A gem that is slotted into an item.

The item in question must be a crafted item, as only crafted items can have slotted gems.

Args:
- ItemId: The unique identifier of the item.
- GemId: The unique identifier of the gem that is slotted into the item.
*%

%*! effective_stat(Stat, Value)

The aggregated value of a stat.

This is calculated by summing up the effective bonuses of all chosen items for the stat, 
and applying any caps that may exist for the stat.

Args:
- Stat: The stat for which the effective value is calculated.
- Value: The aggregated value of the stat after applying bonuses and caps.
*%

%*! dynamic_cap(Stat, Cap)

The dynamic cap for a stat.

This is calculated by taking the base cap of the stat and adding any aggregated overcap.

Args:
- Stat: The stat for which the dynamic cap is calculated.
- Cap: The value of the dynamic cap for the stat.
*%

%*! chosen_bonus(ItemId, Stat, Value)

The effective bonus of a chosen item for a specific stat.

This is used during the aggregation of the bonuses of all chosen items for a stat.

Args:
- ItemId: The unique identifier of the item that is chosen for a slot.
- Stat: The stat for which the effective bonus is calculated.
*%

%*! higher_gem_cost_exists(ItemId, Cost)

In the given item, there exists a gem with a higher cost than the one specified.

Args:
- ItemId: The unique identifier of the item.
- Cost: The cost of the gem that is being compared against.
*%

%*! highest_gem_cost(ItemId, Cost)

The highest gem cost in the given item.

This is used because in DAoC crafting the cost for the most expensive gem is applied twice.

Args:
- ItemId: The unique identifier of the item.
- Cost: The cost of the most expensive gem in the item.
*%