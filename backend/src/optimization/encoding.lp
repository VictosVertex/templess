% Choose an item for each slot.
% We do not pick weapons yet.
1{slot_chosen(SLOT, ITEM):item(ITEM, TYPE, _)}1 :- slot(SLOT,TYPE), SLOT > 13.

% Translate item bonuses to effective bonuses.
% This is done for mapping acuity to the respective actuity stat of the class.
effective_bonus(ITEM, STAT, VALUE) :- 
    item_bonus(ITEM, STAT, VALUE).

effective_bonus(ITEM, STAT, VALUE) :- 
    item_bonus(ITEM, acuity, VALUE),
    class_acuity(STAT).

effective_bonus(ITEM, CAP_STAT, VALUE) :- 
    item_bonus(ITEM, acuity_cap, VALUE),
    class_acuity(BASE_STAT),
    stat_base(CAP_STAT, BASE_STAT).

effective_bonus(ITEM, STAT, VALUE) :- 
    item_bonus(ITEM, all_magic_skills, VALUE),
    class_skill_line(magic_skills,STAT).

{slotted_gem(ITEM,ID):gem(ID,STAT,_,_,_)} = 4 :- slot_chosen(SLOT,ITEM), crafted_item(ITEM).

% Translate gem bonuses per item to effective bonuses.
effective_bonus(ITEM, STAT, VALUE) :-
    slotted_gem(ITEM,ID),
    gem(ID,STAT,_,VALUE,_).

% Calculate stat caps
dynamic_cap(STAT, BASE_CAP + OVER_CAP) :-
    stat(STAT, _, BASE_CAP),
    stat_cap(STAT, CAP_STAT),
    effective_stat(CAP_STAT, OVER_CAP).

dynamic_cap(STAT, BASE_CAP) :-
    stat(STAT, _, BASE_CAP),
    not stat_cap(STAT, _).

% Aggregate effective stats
chosen_bonus(ITEM, STAT,VALUE) :- slot_chosen(_,ITEM), effective_bonus(ITEM, STAT, VALUE).

effective_stat(STAT, CAP) :-
    preference(STAT, _, _),
    stat_baseline(STAT, BASELINE),
    dynamic_cap(STAT, CAP),
    #sum { VALUE, ITEM : chosen_bonus(ITEM, STAT,VALUE) } >= CAP - BASELINE.

effective_stat(STAT, TOTAL + BASELINE) :-
    preference(STAT, _, _),
    stat_baseline(STAT, BASELINE),
    dynamic_cap(STAT, CAP),
    TOTAL = #sum { VALUE, ITEM : chosen_bonus(ITEM, STAT,VALUE) },
    TOTAL < CAP - BASELINE.

% Symmetry breaking for rings and bracers
:- slot_chosen(33,ITEM_1), slot_chosen(34,ITEM_2), ITEM_1 > ITEM_2.
:- slot_chosen(35,ITEM_1), slot_chosen(36,ITEM_2), ITEM_1 > ITEM_2.

% It cannot be the case that the total cost of gems in a crafted item 
% is not in the 360-375 range. This corresponds to 36.0 - 37.5 Imbue Points.
:- slot_chosen(SLOT,ITEM), crafted_item(ITEM), not 360 < #sum { COST, ID:slotted_gem(ITEM,ID),gem(ID,_,_,_,COST)} < 375.

% It cannot be the case that there are more than 3 or less than 1 crafted items.
:- not 1 #count { ITEM : slot_chosen(_,ITEM), crafted_item(ITEM) } 3.

% It cannot be the case that a preference of 10 is not met.
:- effective_stat(STAT, VALUE), preference(STAT, MIN, 10), VALUE < MIN.

% It cannot be the case that two gems of the same stat appear on the same crafted item
:- slotted_gem(ITEM,ID_1), slotted_gem(ITEM,ID_2), gem(ID_1,STAT,_,_,_), gem(ID_2,STAT,_,_,_), ID_1 < ID_2.

% Symmetry breaking of the same gems with the same values
:- slotted_gem(ITEM_1, ID_1), 
   slotted_gem(ITEM_2, ID_2), 
   gem(ID_1, STAT_1, _, VALUE, _), 
   gem(ID_2, STAT_2, _, VALUE, _), 
   stat_gem_category(STAT_1, CATEGORY),
   stat_gem_category(STAT_2, CATEGORY),
   ITEM_1 < ITEM_2,
   STAT_1 > STAT_2.

#maximize { V * UTILITY * WEIGHT@1, STAT : effective_stat(STAT, V), preference(STAT, _, WEIGHT), stat(STAT, UTILITY, _) }.

#show slot_chosen/2.
#show effective_stat/2.
#show slotted_gem/2.