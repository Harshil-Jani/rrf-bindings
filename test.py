import ranx_module

# Usage example
list1 = ["B", "C", "A"]
list2 = ["C", "A", "B"]
list3 = ["C", "A", "B"]

# Combine lists
combined_lists = [list1, list2, list3]

# Perform Reciprocal Rank Fusion
fused_ranking = ranx_module.reciprocal_rank_fusion(combined_lists)

# Print the fused ranking
print("Fused Ranking:")
for rank, (item, score) in enumerate(fused_ranking):
    print(f"{rank + 1}. {item}: {score}")
