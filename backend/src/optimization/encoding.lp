
% Choose an item for each slot that does not have an item already.
% We do not pick weapons yet.
1{slot_chosen(SLOT, ITEM):item(ITEM, TYPE, _)}1 :- slot(SLOT,TYPE), SLOT > 13, not slot_taken(SLOT, _).

% Symmetry breaking for rings and bracers
:- slot_chosen(33,ITEM_1), slot_chosen(34,ITEM_2), ITEM_1 > ITEM_2.
:- slot_chosen(35,ITEM_1), slot_chosen(36,ITEM_2), ITEM_1 > ITEM_2.

% Taken and chosen slots are assigned slots.
slot_assigned(SLOT,ITEM) :- slot_taken(SLOT,ITEM).
slot_assigned(SLOT,ITEM) :- slot_chosen(SLOT,ITEM).

% Calculate the raw total of a stat by summing the bonuses from all assigned items.
% Only consider stats that have preferences, as those are the only ones that contribute to utility.
stat_raw_total(STAT, TOTAL) :-
    preference(STAT,_,_),
    TOTAL = #sum {VALUE, ITEM:slot_assigned(_,ITEM), item_bonus(ITEM, STAT, VALUE)}.


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

:- preference(STAT, MIN, _), MIN > 0, effective_stat(STAT, V), V < MIN.

#maximize { V * WEIGHT@1, STAT : effective_stat(STAT, V), preference(STAT, _, WEIGHT) }.

#show slot_chosen/2.
#show effective_stat/2.