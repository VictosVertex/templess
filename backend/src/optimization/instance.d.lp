%%%%%%%%%%%%%%%%%%%%%%%%%%
% Class
%%%%%%%%%%%%%%%%%%%%%%%%%%

%*! class_skill_line(Category, Line)

A skill line from a category that is relevant for the class.

Args:
- Category: The category of the skill line (e.g., "MagicSkills", "MeleeSkills").
- Line: The specific skill line (e.g., "Smiting", "Sword", "Fire").
*%

%*! class_acuity(Stat)

The stat that is considered an acuity stat for the class. 

This stat is then also increased by items that give acuity bonuses.

Args:
- Stat: The stat that is considered an acuity stat for the class.
*%

%%%%%%%%%%%%%%%%%%%%%%%%%%
% Selections
%%%%%%%%%%%%%%%%%%%%%%%%%%

%*! slot(Id, Name)

A slot to equip an item. Each slot can hold one item.

Args:
- Id: A unique identifier for the slot.
- Name: The name of the slot (e.g., "head", "chest", "legs").
*%

%*! preference(Stat, Min, Weight)

A preference for a given stat expressed by a minimum target value and a weight.

Args:
- Stat: The stat for which the preference is defined.
- Min: The minimum target value for the stat. The optimization will try to meet or exceed this value.
- Weight: The importance of meeting the minimum target value. Higher weights indicate a stronger preference for meeting the target.
*%

%*! stat_baseline(Stat, Value).

The baseline value for a stat that is already provided by user-selected items.

Args:
- Stat: The stat for which the baseline is defined.
- Value: The baseline value for the stat.
*%

%%%%%%%%%%%%%%%%%%%%%%%%%%
% Stats
%%%%%%%%%%%%%%%%%%%%%%%%%%

%*! stat(Name, Utility, BaseCap)
A stat that can be provided by items and gems.

Args:
- Name: The name of the stat (e.g., "strength", "dexterity").
- Utility: A measure of how useful the stat is for the class. This can be used for optimization purposes.
- BaseCap: The base cap for the stat, which can be increased by cap stats.
*%

%*! stat_gem_category(Stat, Category)

The category of gems a stat belongs to. 

This is used for symmetry breaking to ensure that gems of the
same category are ordered consistently across items.

Args:
- Stat: The stat for which the gem category is defined.
- Category: The category of gems the stat belongs to. 
*%

%*! stat_cap(Stat, CapStat)

A stat that increases the cap of another stat.

Args:
- Stat: The stat for which the cap is increased.
- CapStat: The stat that provides the cap increase.
*%

%*! stat_base(CapStat, Stat)

A stat that provides a baseline value for another stat.

Args:
- CapStat: The stat that provides the baseline value.
- Stat: The stat for which the baseline value is provided.
*%

%%%%%%%%%%%%%%%%%%%%%%%%%%
% Items / Crafting
%%%%%%%%%%%%%%%%%%%%%%%%%%

%*! item(Id, Slot, Name)
An item that can be equipped in a slot.
Args:
- Id: A unique identifier for the item.
- Slot: The slot in which the item can be equipped (e.g., "head", "chest").
- Name: The name of the item.
*%

%*! item_bonus(ItemId, Stat, Value)

A bonus provided by an item.

Args:
- ItemId: The unique identifier of the item providing the bonus.
- Stat: The stat that the bonus applies to.
- Value: The value of the bonus provided by the item.
*%
%*! crafted_item(ItemId)

An item that can be crafted.

Only items that can be crafted can have gems slotted into them.

Args:
- ItemId: The unique identifier of the crafted item.
*%

%*! gem(Id, Stat, Tier, Value, Cost)

A gem that can be slotted into an item.

Args:
- Id: A unique identifier for the gem.
- Stat: The stat that the gem provides a bonus to.
- Tier: The tier of the gem.
- Value: The bonus value that the gem provides to the stat.
- Cost: The impue point cost of the gem.
*%
