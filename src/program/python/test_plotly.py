# -*- coding:UTF-8 -*-
import pandas as pd
import os
from plotly.subplots import make_subplots
import plotly.graph_objects as go

cur_dir = os.path.split(os.path.realpath(__file__))[0]
EXCEL1 = pd.read_excel('{}/w800.xls'.format(cur_dir))

fig = make_subplots(rows=2, cols=1)

fig.append_trace(go.Scatter(
    y=EXCEL1['bl_rssi'],
    name='bl_rssi',
), row=1, col=1)


fig.append_trace(go.Scatter(
    y=EXCEL1['ap_rssi'],
    name='ap_rssi',
), row=2, col=1)

fig.show()
