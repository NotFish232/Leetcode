#![allow(
    clippy::needless_range_loop,
    clippy::if_same_then_else,
    clippy::collapsible_else_if
)]
#![cfg_attr(rustfmt, rustfmt_skip)]

pub mod stubs;

// mod_start
#[path = "bin/1.two-sum.rs"]
mod s_two_sum;
#[path = "bin/2.add-two-numbers.rs"]
mod s_add_two_numbers;
#[path = "bin/3.longest-substring-without-repeating-characters.rs"]
mod s_longest_substring_without_repeating_characters;
#[path = "bin/4.median-of-two-sorted-arrays.rs"]
mod s_median_of_two_sorted_arrays;
#[path = "bin/5.longest-palindromic-substring.rs"]
mod s_longest_palindromic_substring;
#[path = "bin/6.zigzag-conversion.rs"]
mod s_zigzag_conversion;
#[path = "bin/7.reverse-integer.rs"]
mod s_reverse_integer;
#[path = "bin/8.string-to-integer-atoi.rs"]
mod s_string_to_integer_atoi;
#[path = "bin/9.palindrome-number.rs"]
mod s_palindrome_number;
#[path = "bin/10.regular-expression-matching.rs"]
mod s_regular_expression_matching;
#[path = "bin/11.container-with-most-water.rs"]
mod s_container_with_most_water;
#[path = "bin/12.integer-to-roman.rs"]
mod s_integer_to_roman;
#[path = "bin/13.roman-to-integer.rs"]
mod s_roman_to_integer;
#[path = "bin/14.longest-common-prefix.rs"]
mod s_longest_common_prefix;
#[path = "bin/15.3sum.rs"]
mod s_3sum;
#[path = "bin/16.3sum-closest.rs"]
mod s_3sum_closest;
#[path = "bin/17.letter-combinations-of-a-phone-number.rs"]
mod s_letter_combinations_of_a_phone_number;
#[path = "bin/18.4sum.rs"]
mod s_4sum;
#[path = "bin/19.remove-nth-node-from-end-of-list.rs"]
mod s_remove_nth_node_from_end_of_list;
#[path = "bin/20.valid-parentheses.rs"]
mod s_valid_parentheses;
#[path = "bin/21.merge-two-sorted-lists.rs"]
mod s_merge_two_sorted_lists;
#[path = "bin/22.generate-parentheses.rs"]
mod s_generate_parentheses;
#[path = "bin/23.merge-k-sorted-lists.rs"]
mod s_merge_k_sorted_lists;
#[path = "bin/24.swap-nodes-in-pairs.rs"]
mod s_swap_nodes_in_pairs;
#[path = "bin/25.reverse-nodes-in-k-group.rs"]
mod s_reverse_nodes_in_k_group;
#[path = "bin/26.remove-duplicates-from-sorted-array.rs"]
mod s_remove_duplicates_from_sorted_array;
#[path = "bin/27.remove-element.rs"]
mod s_remove_element;
#[path = "bin/28.find-the-index-of-the-first-occurrence-in-a-string.rs"]
mod s_find_the_index_of_the_first_occurrence_in_a_string;
#[path = "bin/29.divide-two-integers.rs"]
mod s_divide_two_integers;
#[path = "bin/30.substring-with-concatenation-of-all-words.rs"]
mod s_substring_with_concatenation_of_all_words;
#[path = "bin/31.next-permutation.rs"]
mod s_next_permutation;
#[path = "bin/32.longest-valid-parentheses.rs"]
mod s_longest_valid_parentheses;
#[path = "bin/33.search-in-rotated-sorted-array.rs"]
mod s_search_in_rotated_sorted_array;
#[path = "bin/34.find-first-and-last-position-of-element-in-sorted-array.rs"]
mod s_find_first_and_last_position_of_element_in_sorted_array;
#[path = "bin/35.search-insert-position.rs"]
mod s_search_insert_position;
#[path = "bin/36.valid-sudoku.rs"]
mod s_valid_sudoku;
#[path = "bin/38.count-and-say.rs"]
mod s_count_and_say;
#[path = "bin/39.combination-sum.rs"]
mod s_combination_sum;
#[path = "bin/41.first-missing-positive.rs"]
mod s_first_missing_positive;
#[path = "bin/42.trapping-rain-water.rs"]
mod s_trapping_rain_water;
#[path = "bin/43.multiply-strings.rs"]
mod s_multiply_strings;
#[path = "bin/45.jump-game-ii.rs"]
mod s_jump_game_ii;
#[path = "bin/46.permutations.rs"]
mod s_permutations;
#[path = "bin/47.permutations-ii.rs"]
mod s_permutations_ii;
#[path = "bin/48.rotate-image.rs"]
mod s_rotate_image;
#[path = "bin/49.group-anagrams.rs"]
mod s_group_anagrams;
#[path = "bin/55.jump-game.rs"]
mod s_jump_game;
#[path = "bin/56.merge-intervals.rs"]
mod s_merge_intervals;
#[path = "bin/66.plus-one.rs"]
mod s_plus_one;
#[path = "bin/75.sort-colors.rs"]
mod s_sort_colors;
#[path = "bin/94.binary-tree-inorder-traversal.rs"]
mod s_binary_tree_inorder_traversal;
#[path = "bin/121.best-time-to-buy-and-sell-stock.rs"]
mod s_best_time_to_buy_and_sell_stock;
#[path = "bin/123.best-time-to-buy-and-sell-stock-iii.rs"]
mod s_best_time_to_buy_and_sell_stock_iii;
#[path = "bin/128.longest-consecutive-sequence.rs"]
mod s_longest_consecutive_sequence;
#[path = "bin/139.word-break.rs"]
mod s_word_break;
#[path = "bin/144.binary-tree-preorder-traversal.rs"]
mod s_binary_tree_preorder_traversal;
#[path = "bin/145.binary-tree-postorder-traversal.rs"]
mod s_binary_tree_postorder_traversal;
#[path = "bin/162.find-peak-element.rs"]
mod s_find_peak_element;
#[path = "bin/164.maximum-gap.rs"]
mod s_maximum_gap;
#[path = "bin/165.compare-version-numbers.rs"]
mod s_compare_version_numbers;
#[path = "bin/174.dungeon-game.rs"]
mod s_dungeon_game;
#[path = "bin/188.best-time-to-buy-and-sell-stock-iv.rs"]
mod s_best_time_to_buy_and_sell_stock_iv;
#[path = "bin/198.house-robber.rs"]
mod s_house_robber;
#[path = "bin/200.number-of-islands.rs"]
mod s_number_of_islands;
#[path = "bin/204.count-primes.rs"]
mod s_count_primes;
#[path = "bin/206.reverse-linked-list.rs"]
mod s_reverse_linked_list;
#[path = "bin/213.house-robber-ii.rs"]
mod s_house_robber_ii;
#[path = "bin/215.kth-largest-element-in-an-array.rs"]
mod s_kth_largest_element_in_an_array;
#[path = "bin/239.sliding-window-maximum.rs"]
mod s_sliding_window_maximum;
#[path = "bin/300.longest-increasing-subsequence.rs"]
mod s_longest_increasing_subsequence;
#[path = "bin/307.range-sum-query-mutable.rs"]
mod s_range_sum_query_mutable;
#[path = "bin/309.best-time-to-buy-and-sell-stock-with-cooldown.rs"]
mod s_best_time_to_buy_and_sell_stock_with_cooldown;
#[path = "bin/368.largest-divisible-subset.rs"]
mod s_largest_divisible_subset;
#[path = "bin/374.guess-number-higher-or-lower.rs"]
mod s_guess_number_higher_or_lower;
#[path = "bin/403.frog-jump.rs"]
mod s_frog_jump;
#[path = "bin/413.arithmetic-slices.rs"]
mod s_arithmetic_slices;
#[path = "bin/446.arithmetic-slices-ii-subsequence.rs"]
mod s_arithmetic_slices_ii_subsequence;
#[path = "bin/453.minimum-moves-to-equal-array-elements.rs"]
mod s_minimum_moves_to_equal_array_elements;
#[path = "bin/461.hamming-distance.rs"]
mod s_hamming_distance;
#[path = "bin/462.minimum-moves-to-equal-array-elements-ii.rs"]
mod s_minimum_moves_to_equal_array_elements_ii;
#[path = "bin/475.heaters.rs"]
mod s_heaters;
#[path = "bin/476.number-complement.rs"]
mod s_number_complement;
#[path = "bin/485.max-consecutive-ones.rs"]
mod s_max_consecutive_ones;
#[path = "bin/492.construct-the-rectangle.rs"]
mod s_construct_the_rectangle;
#[path = "bin/524.longest-word-in-dictionary-through-deleting.rs"]
mod s_longest_word_in_dictionary_through_deleting;
#[path = "bin/630.course-schedule-iii.rs"]
mod s_course_schedule_iii;
#[path = "bin/632.smallest-range-covering-elements-from-k-lists.rs"]
mod s_smallest_range_covering_elements_from_k_lists;
#[path = "bin/668.kth-smallest-number-in-multiplication-table.rs"]
mod s_kth_smallest_number_in_multiplication_table;
#[path = "bin/712.minimum-ascii-delete-sum-for-two-strings.rs"]
mod s_minimum_ascii_delete_sum_for_two_strings;
#[path = "bin/717.1-bit-and-2-bit-characters.rs"]
mod s_1_bit_and_2_bit_characters;
#[path = "bin/719.find-k-th-smallest-pair-distance.rs"]
mod s_find_k_th_smallest_pair_distance;
#[path = "bin/763.partition-labels.rs"]
mod s_partition_labels;
#[path = "bin/786.k-th-smallest-prime-fraction.rs"]
mod s_k_th_smallest_prime_fraction;
#[path = "bin/836.rectangle-overlap.rs"]
mod s_rectangle_overlap;
#[path = "bin/865.smallest-subtree-with-all-the-deepest-nodes.rs"]
mod s_smallest_subtree_with_all_the_deepest_nodes;
#[path = "bin/873.length-of-longest-fibonacci-subsequence.rs"]
mod s_length_of_longest_fibonacci_subsequence;
#[path = "bin/889.construct-binary-tree-from-preorder-and-postorder-traversal.rs"]
mod s_construct_binary_tree_from_preorder_and_postorder_traversal;
#[path = "bin/898.bitwise-ors-of-subarrays.rs"]
mod s_bitwise_ors_of_subarrays;
#[path = "bin/952.largest-component-size-by-common-factor.rs"]
mod s_largest_component_size_by_common_factor;
#[path = "bin/1004.max-consecutive-ones-iii.rs"]
mod s_max_consecutive_ones_iii;
#[path = "bin/1028.recover-a-tree-from-preorder-traversal.rs"]
mod s_recover_a_tree_from_preorder_traversal;
#[path = "bin/1079.letter-tile-possibilities.rs"]
mod s_letter_tile_possibilities;
#[path = "bin/1092.shortest-common-supersequence.rs"]
mod s_shortest_common_supersequence;
#[path = "bin/1109.corporate-flight-bookings.rs"]
mod s_corporate_flight_bookings;
#[path = "bin/1123.lowest-common-ancestor-of-deepest-leaves.rs"]
mod s_lowest_common_ancestor_of_deepest_leaves;
#[path = "bin/1125.smallest-sufficient-team.rs"]
mod s_smallest_sufficient_team;
#[path = "bin/1143.longest-common-subsequence.rs"]
mod s_longest_common_subsequence;
#[path = "bin/1220.count-vowels-permutation.rs"]
mod s_count_vowels_permutation;
#[path = "bin/1261.find-elements-in-a-contaminated-binary-tree.rs"]
mod s_find_elements_in_a_contaminated_binary_tree;
#[path = "bin/1284.minimum-number-of-flips-to-convert-binary-matrix-to-zero-matrix.rs"]
mod s_minimum_number_of_flips_to_convert_binary_matrix_to_zero_matrix;
#[path = "bin/1352.product-of-the-last-k-numbers.rs"]
mod s_product_of_the_last_k_numbers;
#[path = "bin/1358.number-of-substrings-containing-all-three-characters.rs"]
mod s_number_of_substrings_containing_all_three_characters;
#[path = "bin/1389.create-target-array-in-the-given-order.rs"]
mod s_create_target_array_in_the_given_order;
#[path = "bin/1415.the-k-th-lexicographical-string-of-all-happy-strings-of-length-n.rs"]
mod s_the_k_th_lexicographical_string_of_all_happy_strings_of_length_n;
#[path = "bin/1433.check-if-a-string-can-break-another-string.rs"]
mod s_check_if_a_string_can_break_another_string;
#[path = "bin/1456.maximum-number-of-vowels-in-a-substring-of-given-length.rs"]
mod s_maximum_number_of_vowels_in_a_substring_of_given_length;
#[path = "bin/1524.number-of-sub-arrays-with-odd-sum.rs"]
mod s_number_of_sub_arrays_with_odd_sum;
#[path = "bin/1665.minimum-initial-energy-to-finish-tasks.rs"]
mod s_minimum_initial_energy_to_finish_tasks;
#[path = "bin/1718.construct-the-lexicographically-largest-valid-sequence.rs"]
mod s_construct_the_lexicographically_largest_valid_sequence;
#[path = "bin/1749.maximum-absolute-sum-of-any-subarray.rs"]
mod s_maximum_absolute_sum_of_any_subarray;
#[path = "bin/1780.check-if-number-is-a-sum-of-powers-of-three.rs"]
mod s_check_if_number_is_a_sum_of_powers_of_three;
#[path = "bin/1863.sum-of-all-subset-xor-totals.rs"]
mod s_sum_of_all_subset_xor_totals;
#[path = "bin/1910.remove-all-occurrences-of-a-substring.rs"]
mod s_remove_all_occurrences_of_a_substring;
#[path = "bin/1976.number-of-ways-to-arrive-at-destination.rs"]
mod s_number_of_ways_to_arrive_at_destination;
#[path = "bin/1980.find-unique-binary-string.rs"]
mod s_find_unique_binary_string;
#[path = "bin/2033.minimum-operations-to-make-a-uni-value-grid.rs"]
mod s_minimum_operations_to_make_a_uni_value_grid;
#[path = "bin/2068.check-whether-two-strings-are-almost-equivalent.rs"]
mod s_check_whether_two_strings_are_almost_equivalent;
#[path = "bin/2108.find-first-palindromic-string-in-the-array.rs"]
mod s_find_first_palindromic_string_in_the_array;
#[path = "bin/2115.find-all-possible-recipes-from-given-supplies.rs"]
mod s_find_all_possible_recipes_from_given_supplies;
#[path = "bin/2140.solving-questions-with-brainpower.rs"]
mod s_solving_questions_with_brainpower;
#[path = "bin/2154.keep-multiplying-found-values-by-two.rs"]
mod s_keep_multiplying_found_values_by_two;
#[path = "bin/2161.partition-array-according-to-given-pivot.rs"]
mod s_partition_array_according_to_given_pivot;
#[path = "bin/2177.find-three-consecutive-integers-that-sum-to-a-given-number.rs"]
mod s_find_three_consecutive_integers_that_sum_to_a_given_number;
#[path = "bin/2206.divide-array-into-equal-pairs.rs"]
mod s_divide_array_into_equal_pairs;
#[path = "bin/2226.maximum-candies-allocated-to-k-children.rs"]
mod s_maximum_candies_allocated_to_k_children;
#[path = "bin/2227.encrypt-and-decrypt-strings.rs"]
mod s_encrypt_and_decrypt_strings;
#[path = "bin/2274.maximum-consecutive-floors-without-special-floors.rs"]
mod s_maximum_consecutive_floors_without_special_floors;
#[path = "bin/2310.sum-of-numbers-with-units-digit-k.rs"]
mod s_sum_of_numbers_with_units_digit_k;
#[path = "bin/2342.max-sum-of-a-pair-with-equal-sum-of-digits.rs"]
mod s_max_sum_of_a_pair_with_equal_sum_of_digits;
#[path = "bin/2366.minimum-replacements-to-sort-the-array.rs"]
mod s_minimum_replacements_to_sort_the_array;
#[path = "bin/2375.construct-smallest-number-from-di-string.rs"]
mod s_construct_smallest_number_from_di_string;
#[path = "bin/2379.minimum-recolors-to-get-k-consecutive-black-blocks.rs"]
mod s_minimum_recolors_to_get_k_consecutive_black_blocks;
#[path = "bin/2401.longest-nice-subarray.rs"]
mod s_longest_nice_subarray;
#[path = "bin/2460.apply-operations-to-an-array.rs"]
mod s_apply_operations_to_an_array;
#[path = "bin/2467.most-profitable-path-in-a-tree.rs"]
mod s_most_profitable_path_in_a_tree;
#[path = "bin/2503.maximum-number-of-points-from-grid-queries.rs"]
mod s_maximum_number_of_points_from_grid_queries;
#[path = "bin/2523.closest-prime-numbers-in-range.rs"]
mod s_closest_prime_numbers_in_range;
#[path = "bin/2529.maximum-count-of-positive-integer-and-negative-integer.rs"]
mod s_maximum_count_of_positive_integer_and_negative_integer;
#[path = "bin/2540.minimum-common-value.rs"]
mod s_minimum_common_value;
#[path = "bin/2551.put-marbles-in-bags.rs"]
mod s_put_marbles_in_bags;
#[path = "bin/2560.house-robber-iv.rs"]
mod s_house_robber_iv;
#[path = "bin/2570.merge-two-2d-arrays-by-summing-values.rs"]
mod s_merge_two_2d_arrays_by_summing_values;
#[path = "bin/2579.count-total-number-of-colored-cells.rs"]
mod s_count_total_number_of_colored_cells;
#[path = "bin/2593.find-score-of-an-array-after-marking-all-elements.rs"]
mod s_find_score_of_an_array_after_marking_all_elements;
#[path = "bin/2594.minimum-time-to-repair-cars.rs"]
mod s_minimum_time_to_repair_cars;
#[path = "bin/2685.count-the-number-of-complete-components.rs"]
mod s_count_the_number_of_complete_components;
#[path = "bin/2719.count-of-integers.rs"]
mod s_count_of_integers;
#[path = "bin/2780.minimum-index-of-a-valid-split.rs"]
mod s_minimum_index_of_a_valid_split;
#[path = "bin/2818.apply-operations-to-maximize-score.rs"]
mod s_apply_operations_to_maximize_score;
#[path = "bin/2828.check-if-a-string-is-an-acronym-of-words.rs"]
mod s_check_if_a_string_is_an_acronym_of_words;
#[path = "bin/2873.maximum-value-of-an-ordered-triplet-i.rs"]
mod s_maximum_value_of_an_ordered_triplet_i;
#[path = "bin/2874.maximum-value-of-an-ordered-triplet-ii.rs"]
mod s_maximum_value_of_an_ordered_triplet_ii;
#[path = "bin/2919.minimum-increment-operations-to-make-array-beautiful.rs"]
mod s_minimum_increment_operations_to_make_array_beautiful;
#[path = "bin/2965.find-missing-and-repeated-values.rs"]
mod s_find_missing_and_repeated_values;
#[path = "bin/3108.minimum-cost-walk-in-weighted-graph.rs"]
mod s_minimum_cost_walk_in_weighted_graph;
#[path = "bin/3116.kth-smallest-amount-with-single-denomination-combination.rs"]
mod s_kth_smallest_amount_with_single_denomination_combination;
#[path = "bin/3121.count-the-number-of-special-characters-ii.rs"]
mod s_count_the_number_of_special_characters_ii;
#[path = "bin/3169.count-days-without-meetings.rs"]
mod s_count_days_without_meetings;
#[path = "bin/3191.minimum-operations-to-make-binary-array-elements-equal-to-one-i.rs"]
mod s_minimum_operations_to_make_binary_array_elements_equal_to_one_i;
#[path = "bin/3207.maximum-points-after-enemy-battles.rs"]
mod s_maximum_points_after_enemy_battles;
#[path = "bin/3208.alternating-groups-ii.rs"]
mod s_alternating_groups_ii;
#[path = "bin/3271.hash-divided-string.rs"]
mod s_hash_divided_string;
#[path = "bin/3273.minimum-amount-of-damage-dealt-to-bob.rs"]
mod s_minimum_amount_of_damage_dealt_to_bob;
#[path = "bin/3280.convert-date-to-binary.rs"]
mod s_convert_date_to_binary;
#[path = "bin/3306.count-of-substrings-containing-every-vowel-and-k-consonants-ii.rs"]
mod s_count_of_substrings_containing_every_vowel_and_k_consonants_ii;
#[path = "bin/3356.zero-array-transformation-ii.rs"]
mod s_zero_array_transformation_ii;
#[path = "bin/3394.check-if-grid-can-be-cut-into-sections.rs"]
mod s_check_if_grid_can_be_cut_into_sections;
#[path = "bin/3474.lexicographically-smallest-generated-string.rs"]
mod s_lexicographically_smallest_generated_string;
#[path = "bin/3490.count-beautiful-numbers.rs"]
mod s_count_beautiful_numbers;
// mod_end
