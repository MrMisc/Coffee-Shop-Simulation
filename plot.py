import cv2
import json
from matplotlib import rc
from matplotlib.font_manager import FontProperties
from scipy.stats import ks_2samp
import scipy.stats as stats  
import os
import seaborn as sns
import matplotlib
import matplotlib.pyplot as plt
import numpy as np
import subprocess
sales, influx, interest, WaitTable, InCourt, sales2, influx2, interest2, WaitTable2, InCourt2 = ([] for _ in range(10)) #First set is chope, second set is no chope
TimeSpent_1 = []
TimeSpent_2 = []
count = -1

while True:
    try:
        DAT_numbers= input().split(" v1")
        DAT = DAT_numbers[0].split(" ")
        sales.append(float(DAT[0]))
        influx.append(float(DAT[1]))
        interest.append(float(DAT[2]))
        WaitTable.append(float(DAT[3]))
        InCourt.append(float(DAT[4]))
        sales2.append(float(DAT[5]))
        influx2.append(float(DAT[6]))
        interest2.append(float(DAT[7]))
        WaitTable2.append(float(DAT[8]))
        InCourt2.append(float(DAT[9]))
        vectors = DAT_numbers[1].split(" v2")            
        TimeSpent_1.append(vectors[0])
        TimeSpent_2.append(vectors[1])
    except:break



print(TimeSpent_1[2])
print(TimeSpent_2[2])


# print(f"First is of type {type(TimeSpent_1[2])}")
# print(f"2nd is of type {type(TimeSpent_2[2])}")

Times1 = []
Times2 = []
for i in TimeSpent_1:
    i = i.replace(" [","").replace("] ","")
    for j in i.split(", "):
        j = j.replace("]","")
        j = j.replace("[","")
        try:Times1.append(float(j))
        except:pass

for i in TimeSpent_2:
    i = i.replace(" [","").replace("] ","")
    for j in i[2:-1].split(", "):
        j = j.replace("]","")
        j = j.replace("[","")
        try:Times2.append(float(j))
        except:pass


#Data plotting
plt.clf()
fig = plt.figure()
plt.style.use('fivethirtyeight') 
plt.figure(figsize=(20,10))
alphaamt = max(max(Times1), max(Times2))
# # print(type(alphaamt))
bins = np.linspace(0, max(Times1), int(len(Times1)**.5))
rc('font',**{'family':'Yu Gothic', 'size': 16})
try:
    fit_alpha, fit_loc, fit_beta=stats.gamma.fit(Times1)
    fit_alpha2, fit_loc2, fit_beta2=stats.gamma.fit(Times2)
    plt.text(int(0.5*max(max(Times1),max(Times2))),10,"KS sample comparison: \n"+(str(ks_2samp(Times1, Times2)).split('(')[1].replace(")","").replace(",","\n"))+ "\n\n"+ f"Simulation 1 Gamma Fit for {fit_alpha} shape and {fit_beta} rate \n"+" " + (str(stats.kstest(data, 'gamma', args=(fit_alpha,fit_loc, fit_beta)))) + "\n\n"+ f"Simulation 2 Gamma Fit for {fit_alpha2} shape and {fit_beta2} rate \n"+" " + (str(stats.kstest(Times2, 'gamma', args=(fit_alpha2,fit_loc2, fit_beta2)))), style = "italic", bbox={'facecolor': 'black', 'alpha': 0.1, 'pad': 10})
except:pass
plt.hist([Times1, Times2], bins, alpha=0.5, label=[f'Something']) 
plt.xlabel("Time Spent in Court", color = 'black')
plt.ylabel("Frequency", color = 'black')
plt.title("Time Distribution", color = 'black')
plt.legend(loc='upper right')
plt.savefig(fname='plot2')


# plt.clf()
rc('font',**{'family':'Yu Gothic', 'size': 16})
fig = plt.figure()
plt.style.use('fivethirtyeight') 
plt.figure(figsize=(20,10))
ax = sns.kdeplot(Times1 , bw_adjust = 0.5 , fill = True, cut = 0, label = "Choping")
sns.kdeplot(Times2 , bw_adjust = 0.5 , fill = True, cut = 0, label = "No Choping ")
plt.xlabel("Time (units)", color = 'black')
plt.ylabel("Frequency", color = 'black')
try:fit_alpha, fit_loc, fit_beta=stats.gamma.fit(Times1)
except:pass
try:fit_alpha2, fit_loc2, fit_beta2=stats.gamma.fit(Times2)
except:pass
try:plt.text(0.5*max(max(Times1),max(Times2)),0.0," "+(str(ks_2samp(Times1, Times2)).split('(')[1].replace(")","").replace(",","\n")) + "\n\n"+ f"Simulation 1 Gamma Fit for {fit_alpha} shape and {fit_beta} rate \n"+" " + (str(stats.kstest(money, 'gamma', args=(fit_alpha,fit_loc, fit_beta)))) + "\n\n"+ f"Simulation 2 Gamma Fit for {fit_alpha2} shape and {fit_beta2} rate \n"+" " + (str(stats.kstest(Times2, 'gamma', args=(fit_alpha2,fit_loc2, fit_beta2)))), style = "italic", bbox={'facecolor': 'black', 'alpha': 0.1, 'pad': 10})
except:pass
# plt.text(0.1,0.0003,  , bbox={'facecolor': 'black', 'alpha': 0.1, 'pad': 10})
# plt.gca().set_xscale("log")
plt.title(f"Time distribution of customer behaviours", color = 'black')
ax.legend(loc='upper right')
plt.savefig(fname='TimeDistribution.png')  



rc('font',**{'family':'Yu Gothic', 'size': 16})
fig = plt.figure()
plt.style.use('fivethirtyeight') 
plt.figure(figsize=(20,10))
ax = sns.kdeplot(sales , bw_adjust = 0.5 , fill = True, cut = 0, label = "Choping")
sns.kdeplot(sales2, bw_adjust = 0.5 , fill = True, cut = 0, label = "No Choping ")
plt.xlabel("Number of fulfilled orders", color = 'black')
plt.ylabel("Frequency", color = 'black')
try:fit_alpha, fit_loc, fit_beta=stats.gamma.fit(sales)
except:pass
try:fit_alpha2, fit_loc2, fit_beta2=stats.gamma.fit(sales2)
except:pass
try:plt.text(0.5*max(max(sales),max(sales2)),0.0," "+(str(ks_2samp(sales, sales2)).split('(')[1].replace(")","").replace(",","\n")) + "\n\n"+ f"Simulation 1 Gamma Fit for {fit_alpha} shape and {fit_beta} rate \n"+" " + (str(stats.kstest(money, 'gamma', args=(fit_alpha,fit_loc, fit_beta)))) + "\n\n"+ f"Simulation 2 Gamma Fit for {fit_alpha2} shape and {fit_beta2} rate \n"+" " + (str(stats.kstest(sales, 'gamma', args=(fit_alpha2,fit_loc2, fit_beta2)))), style = "italic", bbox={'facecolor': 'black', 'alpha': 0.1, 'pad': 10})
except:pass
# plt.text(0.1,0.0003,  , bbox={'facecolor': 'black', 'alpha': 0.1, 'pad': 10})
# plt.gca().set_xscale("log")
plt.title(f"Sales distribution of customer behaviours", color = 'black')
ax.legend(loc='upper right')
plt.savefig(fname='Sales.png')  




rc('font',**{'family':'Yu Gothic', 'size': 16})
fig = plt.figure()
plt.style.use('fivethirtyeight') 
plt.figure(figsize=(20,10))
ax = sns.kdeplot(interest , bw_adjust = 0.5 , fill = True, cut = 0, label = "Choping")
sns.kdeplot(interest2, bw_adjust = 0.5 , fill = True, cut = 0, label = "No Choping ")
plt.xlabel("Number of interested patrons who entered Court", color = 'black')
plt.ylabel("Frequency", color = 'black')
# plt.text(0.1,0.0003,  , bbox={'facecolor': 'black', 'alpha': 0.1, 'pad': 10})
# plt.gca().set_xscale("log")
plt.title(f"Amount of registered interest of passer-bys in Court", color = 'black')
ax.legend(loc='upper right')
plt.savefig(fname='Patrons.png')  








rc('font',**{'family':'Yu Gothic', 'size': 16})
fig = plt.figure()
plt.style.use('fivethirtyeight') 
plt.figure(figsize=(20,10))
ax = sns.kdeplot(WaitTable , bw_adjust = 0.5 , fill = True, cut = 0, label = "Choping")
sns.kdeplot(WaitTable2, bw_adjust = 0.5 , fill = True, cut = 0, label = "No Choping ")
plt.xlabel("Number of customers waiting to get a table", color = 'black')
plt.ylabel("Frequency", color = 'black')
# plt.text(0.1,0.0003,  , bbox={'facecolor': 'black', 'alpha': 0.1, 'pad': 10})
# plt.gca().set_xscale("log")
plt.title(f"Quantity of customers awaited to find a table by end of peak hour", color = 'black')
ax.legend(loc='upper right')
plt.savefig(fname='Waittable_atend.png')  



rc('font',**{'family':'Yu Gothic', 'size': 16})
fig = plt.figure()
plt.style.use('fivethirtyeight') 
plt.figure(figsize=(20,10))
ax = sns.kdeplot(InCourt , bw_adjust = 0.5 , fill = True, cut = 0, label = "Choping")
sns.kdeplot(InCourt2, bw_adjust = 0.5 , fill = True, cut = 0, label = "No Choping ")
plt.xlabel("Number of customers ", color = 'black')
plt.ylabel("Frequency", color = 'black')
# plt.text(0.1,0.0003,  , bbox={'facecolor': 'black', 'alpha': 0.1, 'pad': 10})
# plt.gca().set_xscale("log")
plt.title(f"Quantity of customers still in Court after peak hour", color = 'black')
ax.legend(loc='upper right')
plt.savefig(fname='Remaining_Customers.png')  












# x = np.linspace(1,len(data),len(data))
# plt.clf()
# fig = plt.figure()
# plt.style.use('fivethirtyeight') 
# plt.figure(figsize=(20,10))
# alphaamt = max(tapdata) 
# alphaamt = max(alphaamt,max(tapdata2))
# maxatt = max(int(attempts1), int(attempts2))
# minatt = min(int(attempts1), int(attempts2))
# if alphaamt>60:bins = np.linspace(0, alphaamt,int(alphaamt/maxatt)) #first wrt to the smaller attempt pool
# else:bins = np.linspace(0, alphaamt,int(alphaamt/5))
# bins = np.array(bins)
# plt.hist([tapdata, tapdata2], bins, alpha=0.5, label=[f'First simulation for {str(attempts1) } attempts per session : Method [{str(method1)}]', f'Second simulation for {str(attempts2) } attempts per session: Method [{str(method2)}]']) 
# rc('font',**{'family':'Yu Gothic', 'size': 16})
# try:fit_alpha, fit_loc, fit_beta=stats.gamma.fit(tapdata)
# except:pass
# try:fit_alpha2, fit_loc2, fit_beta2=stats.gamma.fit(tapdata2)
# except:pass
# print(f"Kolmogorov Smirnov test result for tapdata : {ks_2samp(tapdata, tapdata2)}")
# try:plt.text(1,1," "+(str(ks_2samp(tapdata, tapdata2)).split('(')[1].replace(")","").replace(",","\n"))+ "\n\n"+ f"Simulation 1 Gamma Fit for {fit_alpha} shape and {fit_beta} rate \n"+" " + (str(stats.kstest(tapdata, 'gamma', args=(fit_alpha,fit_loc, fit_beta)))) + "\n\n"+ f"Simulation 2 Gamma Fit for {fit_alpha2} shape and {fit_beta2} rate \n"+" " + (str(stats.kstest(tapdata2, 'gamma', args=(fit_alpha2,fit_loc2, fit_beta2)))), style = "italic", bbox={'facecolor': 'black', 'alpha': 0.1, 'pad': 10})
# except:pass
# plt.xlabel("Number of Bernoulli trials taken to success" , color = 'black')
# plt.ylabel("Frequency", color = 'black')
# plt.gca().set_xscale("log")
# plt.title(f"Number of successes every {maxatt} rolls/Bernoulli trials", color = 'black')
# plt.legend(loc='upper right')
# plt.savefig(fname='tapdataplot')
# # plt.show()




# #money plot

# plt.clf()
# rc('font',**{'family':'Yu Gothic', 'size': 16})
# fig = plt.figure()
# plt.style.use('fivethirtyeight') 
# plt.figure(figsize=(20,10))
# ax = sns.kdeplot(money , bw_adjust = 0.5 , fill = True, cut = 0, label = "Simulation 1 ")
# sns.kdeplot(money2 , bw_adjust = 0.5 , fill = True, cut = 0, label = "Simulation 2 ")
# plt.xlabel("Money (units)", color = 'black')
# plt.ylabel("Frequency", color = 'black')
# try:fit_alpha, fit_loc, fit_beta=stats.gamma.fit(money)
# except:pass
# try:fit_alpha2, fit_loc2, fit_beta2=stats.gamma.fit(money2)
# except:pass
# try:plt.text(0.5*max(max(money),max(money2)),0.0," "+(str(ks_2samp(money, money2)).split('(')[1].replace(")","").replace(",","\n")) + "\n\n"+ f"Simulation 1 Gamma Fit for {fit_alpha} shape and {fit_beta} rate \n"+" " + (str(stats.kstest(money, 'gamma', args=(fit_alpha,fit_loc, fit_beta)))) + "\n\n"+ f"Simulation 2 Gamma Fit for {fit_alpha2} shape and {fit_beta2} rate \n"+" " + (str(stats.kstest(money2, 'gamma', args=(fit_alpha2,fit_loc2, fit_beta2)))), style = "italic", bbox={'facecolor': 'black', 'alpha': 0.1, 'pad': 10})
# except:pass
# # plt.text(0.1,0.0003,  , bbox={'facecolor': 'black', 'alpha': 0.1, 'pad': 10})
# # plt.gca().set_xscale("log")
# plt.title(f"Cost distribution of 2 simulations", color = 'black')
# ax.legend(loc='upper right')
# plt.savefig(fname='moneyplot.png')  

# # os._exit(0)
# # subprocess.Popen("py DataEntryPoint.py", shell=True) 



# f = open('duration.json')
# data = json.load(f)
# print("Execution took "+(lambda strset: (str(float(strset)/60.0)+"min") if float(strset)>600.0 else strset + "s")(str(data["Time"])+"."+str(data["Extended Time Details"]*10**-9).split(".")[1]))



# plt.close('all')
# figurine = plt.figure(figsize=(10,9))

# # setting values to rows and column variables
# rows = 2
# columns = 1
  
# # reading images
# Image1 = cv2.imread('tapdataplot.png')
# Image2 = cv2.imread('moneyplot.png')

# figurine.add_subplot(rows, columns, 1)
  
# # showing image
# plt.imshow(Image1)
# plt.axis('off')
# # plt.title("Successes logged by each trial")
  
# # Adds a subplot at the 2nd position
# figurine.add_subplot(rows, columns, 2)
  
# # showing image
# plt.imshow(Image2)
# plt.axis('off')
# # plt.title("Money distribution")

# plt.show()
