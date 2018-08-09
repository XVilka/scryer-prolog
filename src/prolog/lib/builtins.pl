:- op(400, yfx, /).

:- module(builtins, [(=)/2, (+)/2, (*)/2, (-)/2, (/)/2, (/\)/2,
	(\/)/2, (is)/2, (xor)/2, (div)/2, (//)/2, (rdiv)/2, (<<)/2,
	(>>)/2, (mod)/2, (rem)/2, (>)/2, (<)/2, (=\=)/2, (=:=)/2,
	(-)/1, (>=)/2, (=<)/2, (,)/2, (->)/2, (;)/2, (=..)/2, (==)/2,
	(\==)/2, (@=<)/2, (@>=)/2, (@<)/2, (@>)/2, (=@=)/2, (\=@=)/2,
	(:)/2, call_with_inference_limit/3, catch/3,
	setup_call_cleanup/3, throw/1, true/0, false/0]).

/* this is an implementation specific declarative operator used to implement call_with_inference_limit/3
   and setup_call_cleanup/3. switches to the default trust_me and retry_me_else. Indexing choice
   instructions are unchanged. */
:- op(700, fx, non_counted_backtracking).

% arithmetic operators.
:- op(700, xfx, is).
:- op(500, yfx, +).
:- op(500, yfx, -).
:- op(400, yfx, *).
:- op(500, yfx, /\).
:- op(500, yfx, \/).
:- op(500, yfx, xor).
:- op(400, yfx, div).
:- op(400, yfx, //).
:- op(400, yfx, rdiv).
:- op(400, yfx, <<).
:- op(400, yfx, >>).
:- op(400, yfx, mod).
:- op(400, yfx, rem).
:- op(200, fy, -).

% arithmetic comparison operators.
:- op(700, xfx, >).
:- op(700, xfx, <).
:- op(700, xfx, =\=).
:- op(700, xfx, =:=).
:- op(700, xfx, >=).
:- op(700, xfx, =<).

% control.
:- op(700, xfx, =).
:- op(900, fy, \+).
:- op(700, xfx, =..).

% conditional operators.
:- op(1050, xfy, ->).
:- op(1100, xfy, ;).

% term comparison.
:- op(700, xfx, ==).
:- op(700, xfx, \==).
:- op(700, xfx, @=<).
:- op(700, xfx, @>=).
:- op(700, xfx, @<).
:- op(700, xfx, @>).
:- op(700, xfx, =@=).
:- op(700, xfx, \=@=).

% module resolution operator.
:- op(600, xfy, :).

% the maximum arity flag. needs to be replaced with current_prolog_flag(max_arity, MAX_ARITY).
max_arity(63).

% unify.
X = X.

true.

false :- '$fail'.

% control operators.

','(G1, G2) :- '$get_b_value'(B), ','(G1, G2, B).

:- non_counted_backtracking (,)/3.
','(!, ','(G1, G2), B) :- '$set_cp'(B), ','(G1, G2, B).
','(!, !, B) :- '$set_cp'(B).
','(!, G, B) :- '$set_cp'(B), G.
','(G, ','(G2, G3), B) :- !, G, ','(G2, G3, B).
','(G, !, B) :- !, G, '$set_cp'(B).
','(G1, G2, _) :- G1, G2.

;(G1, G2) :- '$get_b_value'(B), ;(G1, G2, B).

:- non_counted_backtracking (;)/3.
;(G1, G4, B) :- compound(G1), G1 = ->(G2, G3), (G2 -> G3 ; '$set_cp'(B), G4).
;(G1, G2, B) :- G1 == !, '$set_cp'(B), call(G2).
;(G1, G2, B) :- G2 == !, call(G2), '$set_cp'(B).
;(G, _, _) :- G.
;(_, G, _) :- G.

G1 -> G2 :- '$get_b_value'(B), ->(G1, G2, B).

:- non_counted_backtracking (->)/3.
->(G1, G2, B) :- G2 == !, call(G1), !, '$set_cp'(B).
->(G1, G2, B) :- call(G1), '$set_cp'(B), call(G2).

% arg.

/* Here is the old, SWI Prolog-imitative arg/3. It has been superseded by an ISO Prolog
 * compliant arg/3 implemented in Rust.

arg(N, Functor, Arg) :- var(N), !, functor(Functor, _, Arity), arg_(N, 1, Arity, Functor, Arg).
arg(N, Functor, Arg) :- integer(N), !, functor(Functor, _, Arity), '$get_arg'(N, Functor, Arg).
arg(N, Functor, Arg) :- throw(error(type_error(integer, N), arg/3)).

arg_(N, N,  N, Functor, Arg)     :- !, '$get_arg'(N, Functor, Arg).
arg_(N, N,  Arity, Functor, Arg) :- '$get_arg'(N, Functor, Arg).
arg_(N, N0, Arity, Functor, Arg) :- N0 < Arity, N1 is N0 + 1, arg_(N, N1, Arity, Functor, Arg).

*/

% univ.

\+ Goal :- call(Goal), !, false.
\+ _.

univ_errors(Term, List, N) :-
    '$skip_max_list'(N, -1, List, R),
    ( var(R)       -> ( var(Term), throw(error(instantiation_error, (=..)/2))      % 8.5.3.3 a)
		      ; true )
    ; R \== []     -> throw(error(type_error(list, List), (=..)/2))                % 8.5.3.3 b)
    ; List = [H|T] -> ( var(H), var(Term), % R == [] => List is a proper list.
		        throw(error(instantiation_error, (=..)/2))                 % 8.5.3.3 c)
		      ; T \== [], nonvar(H), \+ atom(H),
			throw(error(type_error(atom, H), (=..)/2))                 % 8.5.3.3 d)
		      ; compound(H), T == [],
			throw(error(type_error(atomic, H), (=..)/2))               % 8.5.3.3 e)
		      ; var(Term), max_arity(M), N - 1 > M,
			throw(error(representation_error(max_arity), (=..)/2))     % 8.5.3.3 g)
		      ; true )
    ; var(Term)    -> throw(error(domain_error(non_empty_list, List), (=..)/2))    % 8.5.3.3 f)
    ; true ).

Term =.. List :- univ_errors(Term, List, N), univ_worker(Term, List, N).

univ_worker(Term, List, _) :- atomic(Term), !, List = [Term].
univ_worker(Term, [Name|Args], N) :-
    var(Term), !,
    Arity is N-1,
    functor(Term, Name, Arity),
    '$get_args'(Args, Term, 1, Arity).
univ_worker(Term, List, _) :-
    functor(Term, Name, Arity),
    '$get_args'(Args, Term, 1, Arity),
    List = [Name|Args].

'$get_args'(Args, _, _, 0) :-
    !, Args = [].
'$get_args'([Arg], Func, N, N) :-
    !, arg(N, Func, Arg).
'$get_args'([Arg|Args], Func, I0, N) :-
    arg(I0, Func, Arg),
    I1 is I0 + 1,
    '$get_args'(Args, Func, I1, N).

% setup_call_cleanup.

setup_call_cleanup(S, G, C) :- '$get_b_value'(B),
    S, '$set_cp_by_default'(B), '$get_current_block'(Bb),
    ( '$call_with_default_policy'(var(C)) -> throw(error(instantiation_error, setup_call_cleanup/3))
    ; '$call_with_default_policy'(scc_helper(C, G, Bb)) ).

:- non_counted_backtracking scc_helper/3.
scc_helper(C, G, Bb) :-
    '$get_cp'(Cp), '$install_scc_cleaner'(C, NBb), call(G),
    ( '$check_cp'(Cp) -> '$reset_block'(Bb),
			 '$call_with_default_policy'(run_cleaners_without_handling(Cp))
    ; '$call_with_default_policy'(true)
    ; '$reset_block'(NBb), '$fail').
scc_helper(_, _, Bb) :-
    '$reset_block'(Bb), '$get_ball'(Ball),
    '$call_with_default_policy'(run_cleaners_with_handling),
    '$erase_ball',
    '$call_with_default_policy'(throw(Ball)).
scc_helper(_, _, _) :-
    '$get_cp'(Cp),
    '$call_with_default_policy'(run_cleaners_without_handling(Cp)),
    '$fail'.

:- non_counted_backtracking run_cleaners_with_handling/0.
run_cleaners_with_handling :-
    '$get_scc_cleaner'(C), '$get_level'(B),
    '$call_with_default_policy'(catch(C, _, true)),
    '$set_cp_by_default'(B),
    '$call_with_default_policy'(run_cleaners_with_handling).
run_cleaners_with_handling :-
    '$restore_cut_policy'.

:- non_counted_backtracking run_cleaners_without_handling/1.
run_cleaners_without_handling(Cp) :-
    '$get_scc_cleaner'(C), '$get_level'(B), C, '$set_cp_by_default'(B),
    '$call_with_default_policy'(run_cleaners_without_handling(Cp)).
run_cleaners_without_handling(Cp) :-
    '$set_cp_by_default'(Cp), '$restore_cut_policy'.

% call_with_inference_limit

call_with_inference_limit(G, L, R) :-
    '$get_current_block'(Bb),
    '$get_b_value'(B),
    '$call_with_default_policy'(call_with_inference_limit(G, L, R, Bb, B)),
    '$remove_call_policy_check'(B).

:- non_counted_backtracking call_with_inference_limit/5.
call_with_inference_limit(G, L, R, Bb, B) :-
    '$install_new_block'(NBb),
    '$install_inference_counter'(B, L, Count0),
    call(G),
    '$inference_level'(R, B),
    '$remove_inference_counter'(B, Count1),
    '$call_with_default_policy'(is(Diff, L - (Count1 - Count0))),
    '$call_with_default_policy'(end_block(B, Bb, NBb, Diff)).
call_with_inference_limit(_, _, R, Bb, B) :-
    '$reset_block'(Bb),
    '$remove_inference_counter'(B, _),
    ( '$get_ball'(Ball), '$get_level'(Cp), '$set_cp_by_default'(Cp)
    ; '$remove_call_policy_check'(B), '$fail' ),
    '$erase_ball',
    '$call_with_default_policy'(handle_ile(B, Ball, R)).

:- non_counted_backtracking end_block/4.
end_block(_, Bb, NBb, L) :-
    '$clean_up_block'(NBb),
    '$reset_block'(Bb).
end_block(B, Bb, NBb, L) :-
    '$install_inference_counter'(B, L, _),
    '$reset_block'(NBb),
    '$fail'.

:- non_counted_backtracking handle_ile/3.
handle_ile(B, inference_limit_exceeded(B), inference_limit_exceeded) :- !.
handle_ile(B, E, _) :-
    '$remove_call_policy_check'(B),
    '$call_with_default_policy'(throw(E)).

% exceptions.

catch(G,C,R) :- '$get_current_block'(Bb), '$call_with_default_policy'(catch(G,C,R,Bb)).

:- non_counted_backtracking catch/4.
catch(G,C,R,Bb) :-
    '$install_new_block'(NBb), call(G),
    '$call_with_default_policy'(end_block(Bb, NBb)).
catch(G,C,R,Bb) :-
    '$reset_block'(Bb),
    '$get_ball'(Ball),
    '$call_with_default_policy'(handle_ball(Ball, C, R)).

:- non_counted_backtracking end_block/2.
end_block(Bb, NBb) :- '$clean_up_block'(NBb), '$reset_block'(Bb).
end_block(Bb, NBb) :- '$reset_block'(NBb), '$fail'.

:- non_counted_backtracking handle_ball/3.
handle_ball(C, C, R) :- !, '$erase_ball', call(R).
handle_ball(_, _, _) :- '$unwind_stack'.

throw(Ball) :- '$set_ball'(Ball), '$unwind_stack'.
