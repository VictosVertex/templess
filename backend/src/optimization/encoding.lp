#include "encoding.d.lp".
#include "instance.d.lp".

%%%%%%%%%%%%%%%%%%%%%%%%%%
% Generation
%%%%%%%%%%%%%%%%%%%%%%%%%%

% Choose an item for each slot.
% This also selects the bases used for crafting.
% We do not pick weapons yet.
1{equipped_item(SLOT, ITEM):item(ITEM, TYPE, _)}1 :- slot(SLOT,TYPE), SLOT > 13.

% Choose 4 gems for each equipped crafted item.
{equipped_gem(ITEM,ID):gem(ID,STAT,_,_,_)} = 4 :- equipped_item(SLOT,ITEM), crafted_item(ITEM).

%%%%%%%%%%%%%%%%%%%%%%%%%%
% Transformation
%%%%%%%%%%%%%%%%%%%%%%%%%%

resolved_bonus(ITEM, STAT, VALUE) :- 
    item_bonus(ITEM, STAT, VALUE).

resolved_bonus(ITEM, STAT, VALUE) :- 
    item_bonus(ITEM, acuity, VALUE),
    class_acuity(STAT).

resolved_bonus(ITEM, CAP_STAT, VALUE) :- 
    item_bonus(ITEM, acuity_cap, VALUE),
    class_acuity(BASE_STAT),
    stat_to_cap(BASE_STAT,CAP_STAT).

resolved_bonus(ITEM, STAT, VALUE) :- 
    item_bonus(ITEM, all_magic_skills, VALUE),
    class_skill_line(magic_skills,STAT).

resolved_bonus(ITEM, STAT, VALUE) :-
    equipped_gem(ITEM,ID),
    gem(ID,STAT,_,VALUE,_).

% The highest gem cost in an item.
% this is used because in DAoC crafting the cost for the most expensive gem is applied twice.
has_higher_gem_cost(ITEM, COST) :-
    equipped_gem(ITEM, ID_1),
    gem(ID_1, _, _, _, COST),
    equipped_gem(ITEM, ID_2),
    gem(ID_2, _, _, _, OTHER_COST),
    OTHER_COST > COST.

max_gem_cost(ITEM, COST) :-
    equipped_item(_, ITEM),
    crafted_item(ITEM),
    equipped_gem(ITEM, ID),
    gem(ID, _, _, _, COST),
    not has_higher_gem_cost(ITEM, COST).

%%%%%%%%%%%%%%%%%%%%%%%%%%
% Aggregation
%%%%%%%%%%%%%%%%%%%%%%%%%%

% Calculate stat caps
active_cap(STAT, BASE_CAP + OVER_CAP) :-
    stat(STAT, _, BASE_CAP),
    stat_to_cap(STAT, CAP_STAT),
    equipped_stat_total(CAP_STAT, OVER_CAP).

active_cap(STAT, BASE_CAP) :-
    stat(STAT, _, BASE_CAP),
    not stat_to_cap(STAT, _).
  
% Aggregate stats
equipped_bonus(ITEM, STAT,VALUE) :- equipped_item(_,ITEM), resolved_bonus(ITEM, STAT, VALUE).

equipped_stat_total(STAT, CAP) :-
    preference(STAT, _, _),
    stat_baseline(STAT, BASELINE),
    active_cap(STAT, CAP),
    #sum { VALUE, ITEM : equipped_bonus(ITEM, STAT,VALUE) } >= CAP - BASELINE.

equipped_stat_total(STAT, TOTAL + BASELINE) :-
    preference(STAT, _, _),
    stat_baseline(STAT, BASELINE),
    active_cap(STAT, CAP),
    TOTAL = #sum { VALUE, ITEM : equipped_bonus(ITEM, STAT,VALUE) },
    TOTAL < CAP - BASELINE.

%%%%%%%%%%%%%%%%%%%%%%%%%%
% Constraints
%%%%%%%%%%%%%%%%%%%%%%%%%%

% It cannot be the case that the total cost of gems in a crafted item
% is not in the 360-375 range. This corresponds to 36.0 - 37.5 Imbue Points.
%
% Here, we have to consider the highest gem cost twice! We do this by shifting the cost window.
:- equipped_item(SLOT,ITEM), 
    crafted_item(ITEM), 
    max_gem_cost(ITEM, IP), 
    not 360 - IP <= #sum { COST, ID:equipped_gem(ITEM,ID),gem(ID,_,_,_,COST)} <= 375 - IP.

% It cannot be the case that there are more than 2 or less than 1 crafted items.
:- not 1 #count { ITEM : equipped_item(_,ITEM), crafted_item(ITEM) } 2.

% It cannot be the case that two gems of the same stat appear on the same crafted item
:- equipped_gem(ITEM,ID_1), equipped_gem(ITEM,ID_2), gem(ID_1,STAT,_,_,_), gem(ID_2,STAT,_,_,_), ID_1 < ID_2.

% Symmetry breaking for rings and bracers
:- equipped_item(33,ITEM_1), equipped_item(34,ITEM_2), ITEM_1 > ITEM_2.
:- equipped_item(35,ITEM_1), equipped_item(36,ITEM_2), ITEM_1 > ITEM_2.

% Symmetry breaking of the same gems with the same values
:- equipped_gem(ITEM_1, ID_1), 
   equipped_gem(ITEM_2, ID_2), 
   gem(ID_1, STAT_1, _, VALUE, _), 
   gem(ID_2, STAT_2, _, VALUE, _), 
   stat_to_gem_category(STAT_1, CATEGORY),
   stat_to_gem_category(STAT_2, CATEGORY),
   ITEM_1 < ITEM_2,
   STAT_1 > STAT_2.

% Symmetry breaking of the same gems with the same cost
:- equipped_gem(ITEM_1, ID_1), 
   equipped_gem(ITEM_2, ID_2), 
   gem(ID_1, STAT_1, _, _, COST), 
   gem(ID_2, STAT_2, _, _, COST), 
   stat_to_gem_category(STAT_1, CATEGORY),
   stat_to_gem_category(STAT_2, CATEGORY),
   ITEM_1 < ITEM_2,
   STAT_1 > STAT_2.

% It cannot be the case that a preference of 10 is not met.
:- equipped_stat_total(STAT, VALUE), preference(STAT, MIN, 10), VALUE < MIN.

% It cannot be the case that a preference is below 80% of the minimum.
:- equipped_stat_total(STAT, VALUE), preference(STAT, MIN, WEIGHT), VALUE < MIN*4/5, WEIGHT > 0.

%%%%%%%%%%%%%%%%%%%%%%%%%%
% Optimization
%%%%%%%%%%%%%%%%%%%%%%%%%%

% Maximize the weighted utility of the effective stats.
#maximize { V * UTILITY * WEIGHT@2, STAT : equipped_stat_total(STAT, V), preference(STAT, _, WEIGHT), stat(STAT, UTILITY, _)}.

% Minimize the total value of the equipped bonuses to achieve the desired stats.
#minimize { VALUE @1, ITEM, STAT : equipped_bonus(ITEM, STAT, VALUE), preference(STAT, _, _) }.

% Prioritize items that provide bonuses to stats with preferences.
#heuristic equipped_item(SLOT, ITEM) :
    slot(SLOT, TYPE),
    item(ITEM, TYPE, _),
    resolved_bonus(ITEM, STAT, VALUE), VALUE > 0,
    preference(STAT, MIN, WEIGHT), MIN > 0.
[50@1, true]

% Prioritize crafted items with cap increases for stats with preferences.
#heuristic equipped_item(SLOT, ITEM) :
    slot(SLOT, TYPE),
    item(ITEM, TYPE, _),
    crafted_item(ITEM),
    resolved_bonus(ITEM, CAP_STAT, VALUE), VALUE > 0,
    stat_to_cap(STAT, CAP_STAT),
    preference(STAT, MIN, WEIGHT), MIN > 0.
[500@1, true]

#show equipped_item/2.
#show equipped_stat_total/2.
#show equipped_gem/2.
