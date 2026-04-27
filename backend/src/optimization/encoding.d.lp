%*! equipped_item(SlotId, ItemId)

An item that is equipped in a specific slot.

This item may be a fixed or a crafted item.

Args:
- SlotId: The unique identifier of the slot in which the item is equipped.
- ItemId: The unique identifier of the item that is equipped in the slot.
*%

%*! resolved_bonus(ItemId, Stat, Value)

The bonus of an item resolved to a class specific stat.

This is used to combine item bonuses with gem bonuses and to map item
bonuses to the respective stat of the class. For example, `acuity` is a stat
that may or may not increase the value of some specific stat of a class.

Args:
- ItemId: The unique identifier of the item.
- Stat: The stat for which the bonus is resolved.
- Value: The value of the bonus for the stat.
*%

%*! equipped_gem(ItemId, GemId)

A gem that is slotted into an item.

The item in question must be a crafted item, as only crafted items can have slotted gems.

Args:
- ItemId: The unique identifier of the item.
- GemId: The unique identifier of the gem that is slotted into the item.
*%

%*! equipped_stat_total(Stat, Value)

The total resolved bonus for a stat from all equipped items.

This is calculated by summing up the bonuses of all equipped items for the stat, 
and applying any caps that may exist for the stat.

Args:
- Stat: The stat for which the effective value is calculated.
- Value: The aggregated value of the stat after applying bonuses and caps.
*%

%*! active_cap(Stat, Cap)

The active cap for a stat based on the equipped items.

This is calculated by taking the base cap of the stat and adding any aggregated overcap.

Args:
- Stat: The stat for which the cap is calculated.
- Cap: The value of the cap for the stat.
*%

%*! equipped_bonus(ItemId, Stat, Value)

The resolved bonus of an equipped item for a specific stat.

This is used during the aggregation of the bonuses of all equipped items for a stat.

Args:
- ItemId: The unique identifier of the item that is equipped for a slot.
- Stat: The stat for which the effective bonus is calculated.
*%

%*! has_higher_gem_cost(ItemId, Cost)

In the given item, there exists a gem with a higher cost than the one specified.

Args:
- ItemId: The unique identifier of the item.
- Cost: The cost of the gem that is being compared against.
*%

%*! max_gem_cost(ItemId, Cost)

The highest gem cost in the given item.

This is used because in DAoC crafting the cost for the most expensive gem is applied twice.

Args:
- ItemId: The unique identifier of the item.
- Cost: The cost of the most expensive gem in the item.
*%