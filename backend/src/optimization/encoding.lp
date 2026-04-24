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

dynamic_cap(STAT, BASE_CAP + OVER_CAP) :-
    stat(STAT, _, BASE_CAP),
    stat_cap(STAT, CAP_STAT),
    effective_stat(CAP_STAT, OVER_CAP).

dynamic_cap(STAT, BASE_CAP) :-
    stat(STAT, _, BASE_CAP),
    not stat_cap(STAT, _).

effective_stat(STAT, CAP) :-
    preference(STAT, _, _),
    stat_baseline(STAT, BASELINE),
    dynamic_cap(STAT, CAP),
    #sum { VALUE, ITEM : slot_chosen(_,ITEM), effective_bonus(ITEM, STAT, VALUE) } >= CAP - BASELINE.

effective_stat(STAT, TOTAL + BASELINE) :-
    preference(STAT, _, _),
    stat_baseline(STAT, BASELINE),
    dynamic_cap(STAT, CAP),
    TOTAL = #sum { VALUE, ITEM : slot_chosen(_,ITEM), effective_bonus(ITEM, STAT, VALUE) },
    TOTAL < CAP - BASELINE.

stat_hole(STAT, MIN - V):- preference(STAT, MIN, _), MIN > 0, effective_stat(STAT, V), V < MIN.

% It cannot be the case that there is a hole where there is no gem.
:- stat_hole(STAT,_), not gem(_,STAT,_,_,_).

% It cannot be the case that there are more holes than 4 times crafted items.
:- #sum {
       1,STAT : stat_hole(STAT, _) ;
       -4,ITEM : slot_chosen(_,ITEM), crafted_item(ITEM)
   } > 0.

% It cannot be the case that there are more than 3 or less than 1 crafted items.
:- not 1 #count { ITEM : slot_chosen(_,ITEM), crafted_item(ITEM) } 3.

% It cannot be the case that there is a hole that would require 
% more max level gems than there are crafted items
:- stat_hole(STAT,HOLE), #sum { MAX_GEM_VALUE, ITEM : slot_chosen(_,ITEM), crafted_item(ITEM), gem(_,STAT,9,MAX_GEM_VALUE,_) } < HOLE.


#maximize { V * WEIGHT@1, STAT : effective_stat(STAT, V), preference(STAT, _, WEIGHT) }.

#show slot_chosen/2.
#show stat_hole/2.
