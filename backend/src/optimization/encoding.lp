
% Choose an item for each slot.
% We do not pick weapons yet.
1{slot_chosen(SLOT, ITEM):item(ITEM, TYPE, _)}1 :- slot(SLOT,TYPE), SLOT > 13.

% Symmetry breaking for rings and bracers
:- slot_chosen(33,ITEM_1), slot_chosen(34,ITEM_2), ITEM_1 > ITEM_2.
:- slot_chosen(35,ITEM_1), slot_chosen(36,ITEM_2), ITEM_1 > ITEM_2.


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

% Calculate the raw total of a stat by summing the bonuses from all assigned items
% and adding the baseline stat value.
% Only consider stats that have preferences, as those are the only ones that contribute to utility.
stat_raw_total(STAT, TOTAL + BASELINE) :-
    preference(STAT,_,_),
    stat_baseline(STAT, BASELINE),
    TOTAL = #sum {VALUE, ITEM:slot_chosen(_,ITEM), effective_bonus(ITEM, STAT, VALUE)}.


dynamic_cap(STAT, BASE_CAP + OVER_CAP) :-
    stat(STAT, _, BASE_CAP),
    stat_cap(STAT, CAP_STAT),
    effective_stat(CAP_STAT, OVER_CAP).

dynamic_cap(STAT, BASE_CAP) :-
    stat(STAT, _, BASE_CAP),
    not stat_cap(STAT, _).

effective_stat(STAT, TOTAL) :-
    stat_raw_total(STAT, TOTAL),
    dynamic_cap(STAT, CAP),
    TOTAL <= CAP.

effective_stat(STAT, CAP) :-
    stat_raw_total(STAT, TOTAL),
    dynamic_cap(STAT, CAP),
    TOTAL > CAP.

stat_hole(STAT, MIN - V):- preference(STAT, MIN, _), MIN > 0, effective_stat(STAT, V), V < MIN.

stat_hole_count(C) :- C = #count { STAT : stat_hole(STAT, _) }.
crafted_item_count(C) :- C = #count { ITEM : slot_chosen(_,ITEM), crafted_item(ITEM) }.

% It cannot be the case that there is a hole where there is no gem.
:- stat_hole(STAT,_), not gem(_,STAT,_,_).
% It cannot be the case that there are more holes than 4 times crafted items.
:- stat_hole_count(HOLE_COUNT), crafted_item_count(CRAFTED_COUNT), HOLE_COUNT > 4 * CRAFTED_COUNT.
% It cannot be the case that there are more than 3 crafted items.
:- crafted_item_count(CRAFTED_COUNT), CRAFTED_COUNT > 3.
% It cannot be the case that there are no crafted items.
:- crafted_item_count(CRAFTED_COUNT), CRAFTED_COUNT < 1.

#maximize { V * WEIGHT@1, STAT : effective_stat(STAT, V), preference(STAT, _, WEIGHT) }.

#show slot_chosen/2.
#show effective_stat/2.