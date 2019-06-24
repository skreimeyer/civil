#! /usr/env/python3
### A quick and dirty script to generate expressions in rust to insert keys and
### values into a hashmap.


import re

with open("conversion_table.csv",'r') as infile:
    # We want to apply a regex to each line. It might have made more sense to
    # apply the regex to the whole file, but, meh.
    data = [line for line in infile]

# Get rid of all lines that don't actually have conversions in them
data = [n for n in data if not "see table" in n]

# Ok time to replace abbreviations
for i,line in enumerate(data):
    data[i] = re.sub(r'(sq(?=\W))|((?<=\W)sq(?=\W|$))','square ',data[i])
    data[i] = re.sub(r'(cu(?=\W))|((?<=\W)cu(?=\W|$))','cubic ',data[i])
    data[i] = re.sub(r'(ft(?=\W))|((?<=\W)ft(?=\W|$))','foot ',data[i])
    data[i] = re.sub(r'(cm(?=\W))|((?<=\W)cm(?=\W|$))','centimeter ',data[i])
    data[i] = re.sub(r'(km(?=\W))|((?<=\W)km(?=\W|$))','kilometer ',data[i])
    data[i] = re.sub(r'(in(?=\W))|((?<=\W)in(?=\W|$))','inch ',data[i])
    data[i] = re.sub(r'(min(?=\W))|((?<=\W)min(?=\W|$))','minute',data[i])
    data[i] = re.sub(r'(hrs(?=\W))|((?<=\W)hrs(?=\W|$))','hour',data[i])
    data[i] = re.sub(r'((kg)|(kgs)(?=\W))|((?<=\W)(kg)|(kgs)(?=\W|$))','kilogram ',data[i])
    data[i] = re.sub(r'/',' per ',data[i])
    data[i] = re.sub(r'metre','meter',data[i]) # ugh, why do the British do this?
    data[i] = re.sub(r'inches','inch',data[i]) # do this before the regex below
    data[i] = re.sub(r'(?<=[a-z])s(?=,|\s|$)',' ',data[i]) # make singular. This will screw up units that normally end with 's' like 'gauss'
    data[i] = re.sub(r'feet','foot',data[i]) # make singular for feet
    data[i] = re.sub(r' {2,}',' ',data[i]) # collapse all that extra white space we introduced.
    data[i] = re.sub(r' s ',' ',data[i]) # convert orphan 's' characters into single whitespace.

# Write it out to a new csv in case we mess up and need it later
with open("clean_conversion_table.csv",'w') as outfile:
    for line in data:
        outfile.write(line)

# format each line to match our insert statements in units/conversion.rs
with open("formatted_conversions.txt",'w') as outfile:
    for line in data:
        s = line.split(',') # split by comma.
        # Is the conversion even a number at all?
        try:
            float(s[1])
        except:
            continue
        try:
            s[0] = s[0].replace('.','') # get rid of any periods from abbreviations
            s[0] = s[0].strip() # strip any whitespace
            s[2] = s[2].replace('.','')
            s[2] = s[2].strip() # strip the newline character
            expression = 'factors.insert(("{0}","{1}"),{2});\n'.format(s[0],s[2],float(s[1]))
            # We should have 'factors.insert(("meter","foot"),3.2)'
            outfile.write(expression)
        except:
            continue # I can't remember if continue or pass is more pythonic . . .

# fin
