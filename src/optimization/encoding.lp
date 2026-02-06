
% Choose an item for each slot that does not have an item already.
% We do not pick weapons yet.
1{slot_chosen(SLOT, ITEM):item(ITEM, TYPE, _)}1 :- slot(SLOT,TYPE), SLOT > 13, not slot_taken(SLOT, _).

:- slot_chosen(33,ITEM_1), slot_chosen(34,ITEM_2), ITEM_1 > ITEM_2.
:- slot_chosen(35,ITEM_1), slot_chosen(36,ITEM_2), ITEM_1 > ITEM_2.

slot_assigned(SLOT,ITEM) :- slot_taken(SLOT,ITEM).
slot_assigned(SLOT,ITEM) :- slot_chosen(SLOT,ITEM).


stat_raw_total(STAT, TOTAL) :- 
    stat(STAT,_,_), 
    TOTAL = #sum {VALUE, ITEM:slot_assigned(_,ITEM), item_bonus(ITEM, STAT, VALUE)}.

stat_total(STAT, CAP, UTILITY * CAP) :- 
    stat_raw_total(STAT, TOTAL),
    stat(STAT,UTILITY,CAP), 
    TOTAL > CAP,
    not stat_cap(STAT, _).

stat_total(STAT, TOTAL, UTILITY * TOTAL) :- 
    stat_raw_total(STAT, TOTAL),
    stat(STAT,UTILITY,CAP), 
    TOTAL <= CAP,
    not stat_cap(STAT, _).

stat_total(STAT, TOTAL_CAP, UTILITY * TOTAL_CAP) :- 
    stat_raw_total(STAT, BASE_TOTAL),
    stat(STAT,UTILITY,BASE_CAP),
    stat_cap(STAT, CAP_STAT),
    stat_total(CAP_STAT, OVER_CAP, _),
    TOTAL_CAP = BASE_CAP + OVER_CAP,
    BASE_TOTAL > TOTAL_CAP.

stat_total(STAT, BASE_TOTAL, UTILITY * BASE_TOTAL) :- 
    stat_raw_total(STAT, BASE_TOTAL),
    stat(STAT,UTILITY,BASE_CAP),
    stat_cap(STAT, CAP_STAT),
    stat_total(CAP_STAT, OVER_CAP, _),
    TOTAL_CAP = BASE_CAP + OVER_CAP,
    BASE_TOTAL <= TOTAL_CAP.

#maximize {UTILITY: stat_total(_, _, UTILITY)}.

#show slot_assigned/2.
#show stat_raw_total/2.
#show stat_total/3.