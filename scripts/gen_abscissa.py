####################
#
# Convert PHP code to arrays of Gauss-Legendre abscissa and weights because it's
# technically easier than writing a script to actually solve for them.
#
####################
import re

abscissa = []

with open('lgvalues-abscissa (1).php','r') as infile:
    raw_abscissa = infile.readlines()

for i,line in enumerate(raw_abscissa):
    search_result = re.search('(?<=roots\[)\d',line)
    if search_result:
        to_take = int(search_result.group(0))
        tmp_abscissa = []
        for j in range(to_take):
            target_index = i+1+j
            tmp_abscissa.append(float(raw_abscissa[target_index].strip('\t\n,;)')))
        tmp_abscissa.sort()
        [abscissa.append(n) for n in tmp_abscissa]

weights = []

with open('lgvalues-weights.php','r') as infile:
    raw_weights = infile.readlines()

for i,line in enumerate(raw_weights):
    search_result = re.search('(?<=weights\[)\d',line)
    if search_result:
        to_take = int(search_result.group(0))
        tmp_weights = []
        for j in range(to_take):
            target_index = i+1+j
            tmp_weights.append(float(raw_weights[target_index].strip('\t\n,;)')))
        tmp_weights.sort()
        [weights.append(m) for m in tmp_weights]

# And the hacktacular solution
with open('GLweights.rs','w') as outfile:
    outfile.write('pub const ABSCISSA: [f32;{}] = {}'.format(len(abscissa),str(abscissa)))
    outfile.write('\n\n')
    outfile.write('pub const WEIGHTS: [f32;{}] = {}'.format(len(weights),str(weights)))

# fin
