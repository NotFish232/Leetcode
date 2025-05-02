#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(dead_code)]

// mod_start
#[path = "1_two-sum.rs"]
mod s_1_two_sum;
#[path = "2_add-two-numbers.rs"]
mod s_2_add_two_numbers;
#[path = "3_longest-substring-without-repeating-characters.rs"]
mod s_3_longest_substring_without_repeating_characters;
#[path = "4_median-of-two-sorted-arrays.rs"]
mod s_4_median_of_two_sorted_arrays;
#[path = "5_longest-palindromic-substring.rs"]
mod s_5_longest_palindromic_substring;
#[path = "6_zigzag-conversion.rs"]
mod s_6_zigzag_conversion;
#[path = "7_reverse-integer.rs"]
mod s_7_reverse_integer;
#[path = "8_string-to-integer-atoi.rs"]
mod s_8_string_to_integer_atoi;
#[path = "9_palindrome-number.rs"]
mod s_9_palindrome_number;
#[path = "10_regular-expression-matching.rs"]
mod s_10_regular_expression_matching;
#[path = "11_container-with-most-water.rs"]
mod s_11_container_with_most_water;
#[path = "12_integer-to-roman.rs"]
mod s_12_integer_to_roman;
#[path = "13_roman-to-integer.rs"]
mod s_13_roman_to_integer;
#[path = "14_longest-common-prefix.rs"]
mod s_14_longest_common_prefix;
#[path = "15_3sum.rs"]
mod s_15_3sum;
#[path = "16_3sum-closest.rs"]
mod s_16_3sum_closest;
#[path = "17_letter-combinations-of-a-phone-number.rs"]
mod s_17_letter_combinations_of_a_phone_number;
#[path = "18_4sum.rs"]
mod s_18_4sum;
#[path = "19_remove-nth-node-from-end-of-list.rs"]
mod s_19_remove_nth_node_from_end_of_list;
#[path = "20_valid-parentheses.rs"]
mod s_20_valid_parentheses;
#[path = "21_merge-two-sorted-lists.rs"]
mod s_21_merge_two_sorted_lists;
#[path = "22_generate-parentheses.rs"]
mod s_22_generate_parentheses;
#[path = "23_merge-k-sorted-lists.rs"]
mod s_23_merge_k_sorted_lists;
#[path = "24_swap-nodes-in-pairs.rs"]
mod s_24_swap_nodes_in_pairs;
#[path = "25_reverse-nodes-in-k-group.rs"]
mod s_25_reverse_nodes_in_k_group;
#[path = "26_remove-duplicates-from-sorted-array.rs"]
mod s_26_remove_duplicates_from_sorted_array;
#[path = "27_remove-element.rs"]
mod s_27_remove_element;
#[path = "28_find-the-index-of-the-first-occurrence-in-a-string.rs"]
mod s_28_find_the_index_of_the_first_occurrence_in_a_string;
#[path = "29_divide-two-integers.rs"]
mod s_29_divide_two_integers;
#[path = "30_substring-with-concatenation-of-all-words.rs"]
mod s_30_substring_with_concatenation_of_all_words;
#[path = "31_next-permutation.rs"]
mod s_31_next_permutation;
#[path = "32_longest-valid-parentheses.rs"]
mod s_32_longest_valid_parentheses;
#[path = "33_search-in-rotated-sorted-array.rs"]
mod s_33_search_in_rotated_sorted_array;
#[path = "34_find-first-and-last-position-of-element-in-sorted-array.rs"]
mod s_34_find_first_and_last_position_of_element_in_sorted_array;
#[path = "35_search-insert-position.rs"]
mod s_35_search_insert_position;
#[path = "36_valid-sudoku.rs"]
mod s_36_valid_sudoku;
#[path = "38_count-and-say.rs"]
mod s_38_count_and_say;
#[path = "39_combination-sum.rs"]
mod s_39_combination_sum;
#[path = "41_first-missing-positive.rs"]
mod s_41_first_missing_positive;
#[path = "42_trapping-rain-water.rs"]
mod s_42_trapping_rain_water;
#[path = "43_multiply-strings.rs"]
mod s_43_multiply_strings;
#[path = "45_jump-game-ii.rs"]
mod s_45_jump_game_ii;
#[path = "46_permutations.rs"]
mod s_46_permutations;
#[path = "47_permutations-ii.rs"]
mod s_47_permutations_ii;
#[path = "48_rotate-image.rs"]
mod s_48_rotate_image;
#[path = "49_group-anagrams.rs"]
mod s_49_group_anagrams;
#[path = "53_maximum-subarray.rs"]
mod s_53_maximum_subarray;
#[path = "55_jump-game.rs"]
mod s_55_jump_game;
#[path = "56_merge-intervals.rs"]
mod s_56_merge_intervals;
#[path = "66_plus-one.rs"]
mod s_66_plus_one;
#[path = "75_sort-colors.rs"]
mod s_75_sort_colors;
#[path = "94_binary-tree-inorder-traversal.rs"]
mod s_94_binary_tree_inorder_traversal;
#[path = "121_best-time-to-buy-and-sell-stock.rs"]
mod s_121_best_time_to_buy_and_sell_stock;
#[path = "123_best-time-to-buy-and-sell-stock-iii.rs"]
mod s_123_best_time_to_buy_and_sell_stock_iii;
#[path = "128_longest-consecutive-sequence.rs"]
mod s_128_longest_consecutive_sequence;
#[path = "139_word-break.rs"]
mod s_139_word_break;
#[path = "144_binary-tree-preorder-traversal.rs"]
mod s_144_binary_tree_preorder_traversal;
#[path = "145_binary-tree-postorder-traversal.rs"]
mod s_145_binary_tree_postorder_traversal;
#[path = "162_find-peak-element.rs"]
mod s_162_find_peak_element;
#[path = "164_maximum-gap.rs"]
mod s_164_maximum_gap;
#[path = "165_compare-version-numbers.rs"]
mod s_165_compare_version_numbers;
#[path = "174_dungeon-game.rs"]
mod s_174_dungeon_game;
#[path = "188_best-time-to-buy-and-sell-stock-iv.rs"]
mod s_188_best_time_to_buy_and_sell_stock_iv;
#[path = "198_house-robber.rs"]
mod s_198_house_robber;
#[path = "200_number-of-islands.rs"]
mod s_200_number_of_islands;
#[path = "204_count-primes.rs"]
mod s_204_count_primes;
#[path = "206_reverse-linked-list.rs"]
mod s_206_reverse_linked_list;
#[path = "213_house-robber-ii.rs"]
mod s_213_house_robber_ii;
#[path = "215_kth-largest-element-in-an-array.rs"]
mod s_215_kth_largest_element_in_an_array;
#[path = "239_sliding-window-maximum.rs"]
mod s_239_sliding_window_maximum;
#[path = "300_longest-increasing-subsequence.rs"]
mod s_300_longest_increasing_subsequence;
#[path = "307_range-sum-query-mutable.rs"]
mod s_307_range_sum_query_mutable;
#[path = "309_best-time-to-buy-and-sell-stock-with-cooldown.rs"]
mod s_309_best_time_to_buy_and_sell_stock_with_cooldown;
#[path = "368_largest-divisible-subset.rs"]
mod s_368_largest_divisible_subset;
#[path = "374_guess-number-higher-or-lower.rs"]
mod s_374_guess_number_higher_or_lower;
#[path = "403_frog-jump.rs"]
mod s_403_frog_jump;
#[path = "413_arithmetic-slices.rs"]
mod s_413_arithmetic_slices;
#[path = "416_partition-equal-subset-sum.rs"]
mod s_416_partition_equal_subset_sum;
#[path = "446_arithmetic-slices-ii-subsequence.rs"]
mod s_446_arithmetic_slices_ii_subsequence;
#[path = "453_minimum-moves-to-equal-array-elements.rs"]
mod s_453_minimum_moves_to_equal_array_elements;
#[path = "461_hamming-distance.rs"]
mod s_461_hamming_distance;
#[path = "462_minimum-moves-to-equal-array-elements-ii.rs"]
mod s_462_minimum_moves_to_equal_array_elements_ii;
#[path = "475_heaters.rs"]
mod s_475_heaters;
#[path = "476_number-complement.rs"]
mod s_476_number_complement;
#[path = "485_max-consecutive-ones.rs"]
mod s_485_max_consecutive_ones;
#[path = "492_construct-the-rectangle.rs"]
mod s_492_construct_the_rectangle;
#[path = "524_longest-word-in-dictionary-through-deleting.rs"]
mod s_524_longest_word_in_dictionary_through_deleting;
#[path = "630_course-schedule-iii.rs"]
mod s_630_course_schedule_iii;
#[path = "632_smallest-range-covering-elements-from-k-lists.rs"]
mod s_632_smallest_range_covering_elements_from_k_lists;
#[path = "668_kth-smallest-number-in-multiplication-table.rs"]
mod s_668_kth_smallest_number_in_multiplication_table;
#[path = "712_minimum-ascii-delete-sum-for-two-strings.rs"]
mod s_712_minimum_ascii_delete_sum_for_two_strings;
#[path = "717_1-bit-and-2-bit-characters.rs"]
mod s_717_1_bit_and_2_bit_characters;
#[path = "719_find-k-th-smallest-pair-distance.rs"]
mod s_719_find_k_th_smallest_pair_distance;
#[path = "763_partition-labels.rs"]
mod s_763_partition_labels;
#[path = "781_rabbits-in-forest.rs"]
mod s_781_rabbits_in_forest;
#[path = "786_k-th-smallest-prime-fraction.rs"]
mod s_786_k_th_smallest_prime_fraction;
#[path = "836_rectangle-overlap.rs"]
mod s_836_rectangle_overlap;
#[path = "838_push-dominoes.rs"]
mod s_838_push_dominoes;
#[path = "865_smallest-subtree-with-all-the-deepest-nodes.rs"]
mod s_865_smallest_subtree_with_all_the_deepest_nodes;
#[path = "873_length-of-longest-fibonacci-subsequence.rs"]
mod s_873_length_of_longest_fibonacci_subsequence;
#[path = "889_construct-binary-tree-from-preorder-and-postorder-traversal.rs"]
mod s_889_construct_binary_tree_from_preorder_and_postorder_traversal;
#[path = "898_bitwise-ors-of-subarrays.rs"]
mod s_898_bitwise_ors_of_subarrays;
#[path = "952_largest-component-size-by-common-factor.rs"]
mod s_952_largest_component_size_by_common_factor;
#[path = "992_subarrays-with-k-different-integers.rs"]
mod s_992_subarrays_with_k_different_integers;
#[path = "1004_max-consecutive-ones-iii.rs"]
mod s_1004_max_consecutive_ones_iii;
#[path = "1015_smallest-integer-divisible-by-k.rs"]
mod s_1015_smallest_integer_divisible_by_k;
#[path = "1028_recover-a-tree-from-preorder-traversal.rs"]
mod s_1028_recover_a_tree_from_preorder_traversal;
#[path = "1079_letter-tile-possibilities.rs"]
mod s_1079_letter_tile_possibilities;
#[path = "1092_shortest-common-supersequence.rs"]
mod s_1092_shortest_common_supersequence;
#[path = "1109_corporate-flight-bookings.rs"]
mod s_1109_corporate_flight_bookings;
#[path = "1123_lowest-common-ancestor-of-deepest-leaves.rs"]
mod s_1123_lowest_common_ancestor_of_deepest_leaves;
#[path = "1125_smallest-sufficient-team.rs"]
mod s_1125_smallest_sufficient_team;
#[path = "1143_longest-common-subsequence.rs"]
mod s_1143_longest_common_subsequence;
#[path = "1220_count-vowels-permutation.rs"]
mod s_1220_count_vowels_permutation;
#[path = "1261_find-elements-in-a-contaminated-binary-tree.rs"]
mod s_1261_find_elements_in_a_contaminated_binary_tree;
#[path = "1282_group-the-people-given-the-group-size-they-belong-to.rs"]
mod s_1282_group_the_people_given_the_group_size_they_belong_to;
#[path = "1284_minimum-number-of-flips-to-convert-binary-matrix-to-zero-matrix.rs"]
mod s_1284_minimum_number_of_flips_to_convert_binary_matrix_to_zero_matrix;
#[path = "1295_find-numbers-with-even-number-of-digits.rs"]
mod s_1295_find_numbers_with_even_number_of_digits;
#[path = "1352_product-of-the-last-k-numbers.rs"]
mod s_1352_product_of_the_last_k_numbers;
#[path = "1358_number-of-substrings-containing-all-three-characters.rs"]
mod s_1358_number_of_substrings_containing_all_three_characters;
#[path = "1389_create-target-array-in-the-given-order.rs"]
mod s_1389_create_target_array_in_the_given_order;
#[path = "1399_count-largest-group.rs"]
mod s_1399_count_largest_group;
#[path = "1415_the-k-th-lexicographical-string-of-all-happy-strings-of-length-n.rs"]
mod s_1415_the_k_th_lexicographical_string_of_all_happy_strings_of_length_n;
#[path = "1433_check-if-a-string-can-break-another-string.rs"]
mod s_1433_check_if_a_string_can_break_another_string;
#[path = "1456_maximum-number-of-vowels-in-a-substring-of-given-length.rs"]
mod s_1456_maximum_number_of_vowels_in_a_substring_of_given_length;
#[path = "1482_minimum-number-of-days-to-make-m-bouquets.rs"]
mod s_1482_minimum_number_of_days_to_make_m_bouquets;
#[path = "1524_number-of-sub-arrays-with-odd-sum.rs"]
mod s_1524_number_of_sub_arrays_with_odd_sum;
#[path = "1534_count-good-triplets.rs"]
mod s_1534_count_good_triplets;
#[path = "1567_maximum-length-of-subarray-with-positive-product.rs"]
mod s_1567_maximum_length_of_subarray_with_positive_product;
#[path = "1665_minimum-initial-energy-to-finish-tasks.rs"]
mod s_1665_minimum_initial_energy_to_finish_tasks;
#[path = "1718_construct-the-lexicographically-largest-valid-sequence.rs"]
mod s_1718_construct_the_lexicographically_largest_valid_sequence;
#[path = "1749_maximum-absolute-sum-of-any-subarray.rs"]
mod s_1749_maximum_absolute_sum_of_any_subarray;
#[path = "1780_check-if-number-is-a-sum-of-powers-of-three.rs"]
mod s_1780_check_if_number_is_a_sum_of_powers_of_three;
#[path = "1863_sum-of-all-subset-xor-totals.rs"]
mod s_1863_sum_of_all_subset_xor_totals;
#[path = "1910_remove-all-occurrences-of-a-substring.rs"]
mod s_1910_remove_all_occurrences_of_a_substring;
#[path = "1922_count-good-numbers.rs"]
mod s_1922_count_good_numbers;
#[path = "1976_number-of-ways-to-arrive-at-destination.rs"]
mod s_1976_number_of_ways_to_arrive_at_destination;
#[path = "1980_find-unique-binary-string.rs"]
mod s_1980_find_unique_binary_string;
#[path = "2033_minimum-operations-to-make-a-uni-value-grid.rs"]
mod s_2033_minimum_operations_to_make_a_uni_value_grid;
#[path = "2068_check-whether-two-strings-are-almost-equivalent.rs"]
mod s_2068_check_whether_two_strings_are_almost_equivalent;
#[path = "2071_maximum-number-of-tasks-you-can-assign.rs"]
mod s_2071_maximum_number_of_tasks_you_can_assign;
#[path = "2108_find-first-palindromic-string-in-the-array.rs"]
mod s_2108_find_first_palindromic_string_in_the_array;
#[path = "2115_find-all-possible-recipes-from-given-supplies.rs"]
mod s_2115_find_all_possible_recipes_from_given_supplies;
#[path = "2140_solving-questions-with-brainpower.rs"]
mod s_2140_solving_questions_with_brainpower;
#[path = "2145_count-the-hidden-sequences.rs"]
mod s_2145_count_the_hidden_sequences;
#[path = "2154_keep-multiplying-found-values-by-two.rs"]
mod s_2154_keep_multiplying_found_values_by_two;
#[path = "2161_partition-array-according-to-given-pivot.rs"]
mod s_2161_partition_array_according_to_given_pivot;
#[path = "2176_count-equal-and-divisible-pairs-in-an-array.rs"]
mod s_2176_count_equal_and_divisible_pairs_in_an_array;
#[path = "2177_find-three-consecutive-integers-that-sum-to-a-given-number.rs"]
mod s_2177_find_three_consecutive_integers_that_sum_to_a_given_number;
#[path = "2179_count-good-triplets-in-an-array.rs"]
mod s_2179_count_good_triplets_in_an_array;
#[path = "2206_divide-array-into-equal-pairs.rs"]
mod s_2206_divide_array_into_equal_pairs;
#[path = "2226_maximum-candies-allocated-to-k-children.rs"]
mod s_2226_maximum_candies_allocated_to_k_children;
#[path = "2227_encrypt-and-decrypt-strings.rs"]
mod s_2227_encrypt_and_decrypt_strings;
#[path = "2274_maximum-consecutive-floors-without-special-floors.rs"]
mod s_2274_maximum_consecutive_floors_without_special_floors;
#[path = "2302_count-subarrays-with-score-less-than-k.rs"]
mod s_2302_count_subarrays_with_score_less_than_k;
#[path = "2310_sum-of-numbers-with-units-digit-k.rs"]
mod s_2310_sum_of_numbers_with_units_digit_k;
#[path = "2338_count-the-number-of-ideal-arrays.rs"]
mod s_2338_count_the_number_of_ideal_arrays;
#[path = "2342_max-sum-of-a-pair-with-equal-sum-of-digits.rs"]
mod s_2342_max_sum_of_a_pair_with_equal_sum_of_digits;
#[path = "2366_minimum-replacements-to-sort-the-array.rs"]
mod s_2366_minimum_replacements_to_sort_the_array;
#[path = "2375_construct-smallest-number-from-di-string.rs"]
mod s_2375_construct_smallest_number_from_di_string;
#[path = "2379_minimum-recolors-to-get-k-consecutive-black-blocks.rs"]
mod s_2379_minimum_recolors_to_get_k_consecutive_black_blocks;
#[path = "2401_longest-nice-subarray.rs"]
mod s_2401_longest_nice_subarray;
#[path = "2444_count-subarrays-with-fixed-bounds.rs"]
mod s_2444_count_subarrays_with_fixed_bounds;
#[path = "2460_apply-operations-to-an-array.rs"]
mod s_2460_apply_operations_to_an_array;
#[path = "2467_most-profitable-path-in-a-tree.rs"]
mod s_2467_most_profitable_path_in_a_tree;
#[path = "2503_maximum-number-of-points-from-grid-queries.rs"]
mod s_2503_maximum_number_of_points_from_grid_queries;
#[path = "2523_closest-prime-numbers-in-range.rs"]
mod s_2523_closest_prime_numbers_in_range;
#[path = "2529_maximum-count-of-positive-integer-and-negative-integer.rs"]
mod s_2529_maximum_count_of_positive_integer_and_negative_integer;
#[path = "2537_count-the-number-of-good-subarrays.rs"]
mod s_2537_count_the_number_of_good_subarrays;
#[path = "2540_minimum-common-value.rs"]
mod s_2540_minimum_common_value;
#[path = "2551_put-marbles-in-bags.rs"]
mod s_2551_put_marbles_in_bags;
#[path = "2560_house-robber-iv.rs"]
mod s_2560_house_robber_iv;
#[path = "2563_count-the-number-of-fair-pairs.rs"]
mod s_2563_count_the_number_of_fair_pairs;
#[path = "2570_merge-two-2d-arrays-by-summing-values.rs"]
mod s_2570_merge_two_2d_arrays_by_summing_values;
#[path = "2579_count-total-number-of-colored-cells.rs"]
mod s_2579_count_total_number_of_colored_cells;
#[path = "2593_find-score-of-an-array-after-marking-all-elements.rs"]
mod s_2593_find_score_of_an_array_after_marking_all_elements;
#[path = "2594_minimum-time-to-repair-cars.rs"]
mod s_2594_minimum_time_to_repair_cars;
#[path = "2685_count-the-number-of-complete-components.rs"]
mod s_2685_count_the_number_of_complete_components;
#[path = "2719_count-of-integers.rs"]
mod s_2719_count_of_integers;
#[path = "2780_minimum-index-of-a-valid-split.rs"]
mod s_2780_minimum_index_of_a_valid_split;
#[path = "2799_count-complete-subarrays-in-an-array.rs"]
mod s_2799_count_complete_subarrays_in_an_array;
#[path = "2818_apply-operations-to-maximize-score.rs"]
mod s_2818_apply_operations_to_maximize_score;
#[path = "2828_check-if-a-string-is-an-acronym-of-words.rs"]
mod s_2828_check_if_a_string_is_an_acronym_of_words;
#[path = "2843_count-symmetric-integers.rs"]
mod s_2843_count_symmetric_integers;
#[path = "2845_count-of-interesting-subarrays.rs"]
mod s_2845_count_of_interesting_subarrays;
#[path = "2873_maximum-value-of-an-ordered-triplet-i.rs"]
mod s_2873_maximum_value_of_an_ordered_triplet_i;
#[path = "2874_maximum-value-of-an-ordered-triplet-ii.rs"]
mod s_2874_maximum_value_of_an_ordered_triplet_ii;
#[path = "2919_minimum-increment-operations-to-make-array-beautiful.rs"]
mod s_2919_minimum_increment_operations_to_make_array_beautiful;
#[path = "2962_count-subarrays-where-max-element-appears-at-least-k-times.rs"]
mod s_2962_count_subarrays_where_max_element_appears_at_least_k_times;
#[path = "2965_find-missing-and-repeated-values.rs"]
mod s_2965_find_missing_and_repeated_values;
#[path = "2999_count-the-number-of-powerful-integers.rs"]
mod s_2999_count_the_number_of_powerful_integers;
#[path = "3039_apply-operations-to-make-string-empty.rs"]
mod s_3039_apply_operations_to_make_string_empty;
#[path = "3108_minimum-cost-walk-in-weighted-graph.rs"]
mod s_3108_minimum_cost_walk_in_weighted_graph;
#[path = "3116_kth-smallest-amount-with-single-denomination-combination.rs"]
mod s_3116_kth_smallest_amount_with_single_denomination_combination;
#[path = "3121_count-the-number-of-special-characters-ii.rs"]
mod s_3121_count_the_number_of_special_characters_ii;
#[path = "3169_count-days-without-meetings.rs"]
mod s_3169_count_days_without_meetings;
#[path = "3191_minimum-operations-to-make-binary-array-elements-equal-to-one-i.rs"]
mod s_3191_minimum_operations_to_make_binary_array_elements_equal_to_one_i;
#[path = "3207_maximum-points-after-enemy-battles.rs"]
mod s_3207_maximum_points_after_enemy_battles;
#[path = "3208_alternating-groups-ii.rs"]
mod s_3208_alternating_groups_ii;
#[path = "3271_hash-divided-string.rs"]
mod s_3271_hash_divided_string;
#[path = "3272_find-the-count-of-good-integers.rs"]
mod s_3272_find_the_count_of_good_integers;
#[path = "3273_minimum-amount-of-damage-dealt-to-bob.rs"]
mod s_3273_minimum_amount_of_damage_dealt_to_bob;
#[path = "3280_convert-date-to-binary.rs"]
mod s_3280_convert_date_to_binary;
#[path = "3306_count-of-substrings-containing-every-vowel-and-k-consonants-ii.rs"]
mod s_3306_count_of_substrings_containing_every_vowel_and_k_consonants_ii;
#[path = "3356_zero-array-transformation-ii.rs"]
mod s_3356_zero_array_transformation_ii;
#[path = "3375_minimum-operations-to-make-array-values-equal-to-k.rs"]
mod s_3375_minimum_operations_to_make_array_values_equal_to_k;
#[path = "3392_count-subarrays-of-length-three-with-a-condition.rs"]
mod s_3392_count_subarrays_of_length_three_with_a_condition;
#[path = "3394_check-if-grid-can-be-cut-into-sections.rs"]
mod s_3394_check_if_grid_can_be_cut_into_sections;
#[path = "3396_minimum-number-of-operations-to-make-elements-in-array-distinct.rs"]
mod s_3396_minimum_number_of_operations_to_make_elements_in_array_distinct;
#[path = "3474_lexicographically-smallest-generated-string.rs"]
mod s_3474_lexicographically_smallest_generated_string;
#[path = "3490_count-beautiful-numbers.rs"]
mod s_3490_count_beautiful_numbers;
// mod_end