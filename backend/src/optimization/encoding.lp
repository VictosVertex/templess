#include "encoding.d.lp".
#include "instance.d.lp".

%%%%%%%%%%%%%%%%%%%%%%%%%%
% Generation
%%%%%%%%%%%%%%%%%%%%%%%%%%

% Choose an item for each slot.
% This also selects the bases used for crafting.
% We do not pick weapons yet.
{equipped_item(SLOT, ITEM):item(ITEM, TYPE, _)} = 1 :- slot(SLOT,TYPE), SLOT > 13.

% Choose 4 gems for each slot that contains a crafted item.
{equipped_gem(SLOT,ID):gem(ID,STAT,_,_,_)} = 4 :- equipped_item(SLOT,ITEM), crafted_item(ITEM).

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

equipped_bonus(SLOT,STAT,VALUE) :- equipped_item(SLOT,ITEM), resolved_bonus(ITEM, STAT, VALUE).
equipped_bonus(SLOT,STAT,VALUE) :- equipped_gem(SLOT,ID), gem(ID,STAT,_,VALUE,_).

% The highest gem cost in a slot.
% this is used because in DAoC crafting the cost for the most expensive gem is applied twice.
has_higher_gem_cost(SLOT, COST) :-
    equipped_gem(SLOT, ID_1),
    gem(ID_1, _, _, _, COST),
    equipped_gem(SLOT, ID_2),
    gem(ID_2, _, _, _, OTHER_COST),
    OTHER_COST > COST.

max_gem_cost(SLOT, COST) :-
    equipped_item(_, SLOT),
    crafted_item(SLOT),
    equipped_gem(SLOT, ID),
    gem(ID, _, _, _, COST),
    not has_higher_gem_cost(SLOT, COST).

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

equipped_stat_total(STAT, CAP) :-
    preference(STAT, _, _),
    stat_baseline(STAT, BASELINE),
    active_cap(STAT, CAP),
    #sum { VALUE, SLOT : equipped_bonus(SLOT,STAT,VALUE) } >= CAP - BASELINE.

equipped_stat_total(STAT, TOTAL + BASELINE) :-
    preference(STAT, _, _),
    stat_baseline(STAT, BASELINE),
    active_cap(STAT, CAP),
    TOTAL = #sum { VALUE, SLOT : equipped_bonus(SLOT,STAT,VALUE) },
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
    max_gem_cost(SLOT, IP), 
    not 360 - IP <= #sum { COST, ID:equipped_gem(SLOT,ID),gem(ID,_,_,_,COST)} <= 375 - IP.

% It cannot be the case that there are more than 2 or less than 1 crafted items.
:- not 1 #count { ITEM : equipped_item(_,ITEM), crafted_item(ITEM) } 2.

% It cannot be the case that two gems of the same stat appear on the same crafted item
:- equipped_gem(SLOT,ID_1), equipped_gem(SLOT,ID_2), gem(ID_1,STAT,_,_,_), gem(ID_2,STAT,_,_,_), ID_1 < ID_2.

% Symmetry breaking for rings and bracers
:- equipped_item(33,ITEM_1), equipped_item(34,ITEM_2), ITEM_1 > ITEM_2.
:- equipped_item(35,ITEM_1), equipped_item(36,ITEM_2), ITEM_1 > ITEM_2.

% Symmetry breaking of the same gems with the same cost
:- equipped_gem(SLOT_1, ID_1), 
   equipped_gem(SLOT_2, ID_2), 
   gem(ID_1, STAT_1, _, _, COST), 
   gem(ID_2, STAT_2, _, _, COST), 
   stat_to_gem_category(STAT_1, _),
   stat_to_gem_category(STAT_2, _),
   SLOT_1 < SLOT_2,
   STAT_1 > STAT_2.

% It cannot be the case that a preference of 10 is not met.
:- equipped_stat_total(STAT, VALUE), preference(STAT, MIN, 10), VALUE < MIN.

% It cannot be the case that a preference is below 80% of the minimum.
:- equipped_stat_total(STAT, VALUE), preference(STAT, MIN, WEIGHT), VALUE < MIN*4/5, WEIGHT > 0.

% It cannot be the case that a gem is equipped that provides an unnecessarily high stat.
:-  stat_baseline(STAT, BASELINE),
    stat_to_gem_category(STAT, CATEGORY), 
    active_cap(STAT, CAP),
    equipped_gem(_, ID),
    gem(ID, STAT, _, _, _),
    gem_overcap_limit(CATEGORY, LIMIT),
    TOLERANCE = (CAP-BASELINE+LIMIT),
    TOLERANCE < #sum { VALUE, SLOT : equipped_bonus(SLOT, STAT, VALUE) }.

%%%%%%%%%%%%%%%%%%%%%%%%%%
% Optimization
%%%%%%%%%%%%%%%%%%%%%%%%%%

% Minimize the weighted distance to the minimum preference for stats that are below the minimum.
%
% Idea: Prioritize the fillings of minimum preferences.
#minimize { (MIN - VALUE) * UTILITY * WEIGHT @1, STAT : equipped_stat_total(STAT, VALUE), preference(STAT, MIN, WEIGHT), stat(STAT, UTILITY, _), VALUE < MIN, MIN > 0 }.

% Minimize the total value of the equipped bonuses to achieve the desired stats.
%
% Idea: Push down overshoots and provide a goal to "look in other directions"
%
% In practice, this seems to add behaviour that leads to smaller periods 
% where the solver is stuck.
#minimize { VALUE * UTILITY @2, SLOT, STAT : equipped_bonus(SLOT, STAT, VALUE), preference(STAT, _, _), stat(STAT, UTILITY, _) }.


% Maximize the weighted utility of the total stats.
%
% Idea: Still fill up everything after reaching the minimum preferences.
#maximize { VALUE * UTILITY * WEIGHT@3, STAT : equipped_stat_total(STAT, VALUE), preference(STAT, _, WEIGHT), stat(STAT, UTILITY, _) }.

% overflow_penalty_step(20, 1000, 4).
% overflow_penalty_step(15, 500, 3).
% overflow_penalty_step(10, 200, 3).
% overflow_penalty_step(5,  50, 3).

% % % Penalty for stat, resist or hitpoints that are some percentage bigger than the cap.
% :~  stat_baseline(STAT, BASELINE),
%     stat_to_gem_category(STAT, (stat;resists;hitpoints)),
%     stat(STAT, _, BASE_CAP),
%     stat_to_cap(STAT, CAP_STAT),
%     stat(CAP_STAT, _, CAP),
%     % active_cap(STAT, CAP),
%     overflow_penalty_step(P,PENALTY, LEVEL),
%     TOLERANCE = (CAP+BASE_CAP)*(100+P)/100 - BASELINE,
%     TOLERANCE < #sum { VALUE, SLOT : equipped_bonus(SLOT,STAT,VALUE) }. [PENALTY@LEVEL, STAT, P]

%%%%%%%%%%%%%%%%%%%%%%%%%%
% Heuristics
%%%%%%%%%%%%%%%%%%%%%%%%%%

% % Prioritize items with many high utility stats with preferences.
% %
% % Idea: The optimal model will consist of some collection of
% % high utility items.
% %
% % This seems to give good models early but then get hardstuck.
% #heuristic equipped_item(SLOT, ITEM) :
%     slot(SLOT, TYPE),
%     item(ITEM, TYPE, _),
%     resolved_bonus(ITEM, STAT, VALUE), VALUE > 0,
%     stat(STAT, UTILITY, _),
%     VALUE * UTILITY > 1200,
%     preference(STAT, MIN, WEIGHT), MIN > 0.
% [400@2, true]

% % Prioritize items with many stats with preferences.
% %
% % Idea: If an item has low utility stats, we still prefer
% % the ones we want to reach a certain threshold with.
% %
% % Similar to the high utility version this gives good models fast
% % but then gets hardstuck. Usually the models are stightly worse than
% % for the high utility variant.
% %
% % Combining them still results in being stuck for long periods.
% #heuristic equipped_item(SLOT, ITEM) :
%     slot(SLOT, TYPE),
%     item(ITEM, TYPE, _),
%     resolved_bonus(ITEM, STAT, VALUE), VALUE > 0,
%     preference(STAT, MIN, WEIGHT), MIN > 0.
% [100@2, true]

% Prioritize crafted items.
%
% Idea: From experience, most top builds sit around
% two crafted pieces.
% #heuristic equipped_item(SLOT, ITEM) :
%     slot(SLOT, TYPE),
%     item(ITEM, TYPE, _),
%     crafted_item(ITEM),
%     preference(STAT, _, _).
% [1000@2, true]

% % Prioritize crafted items.
% %
% % Idea: From experience, most top builds sit around
% % two crafted pieces.
% #heuristic equipped_item(SLOT, ITEM) :
%     slot(SLOT, TYPE),
%     item(ITEM, TYPE, _),
%     crafted_item(ITEM).
% [500@2, true]


% Prioritize mid tier gems.
%
% Idea: Usually crafted items are used to fill missing resists
% or close medium stat holes. We don't really care about tiny
% or massive gems most of the time.
#heuristic equipped_gem(SLOT, ID) :
    equipped_item(SLOT,ITEM),
    crafted_item(ITEM),
    gem(ID, _, TIER, _, _),
    TIER >= 4, TIER <= 8.
[150@3, true]

#show equipped_item/2.
#show equipped_stat_total/2.
#show equipped_gem(ITEM,ID): equipped_gem(SLOT, ID), equipped_item(SLOT, ITEM).
