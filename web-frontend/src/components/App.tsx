import { createClient, FluenceClient, subscribeToEvent } from '@fluencelabs/fluence';
import React, { useEffect, useState } from 'react';
import { createFilter, getFilterChanges, getTxInfo, relayNode, removeFilter, TxInfo } from 'src/fluence';

import './App.scss';

const intervalMs = 1000;

const App = () => {
    const [client, setClient] = useState<FluenceClient | null>(null);
    const [serviceUrl, setServiceUrl] = useState(
        'https://eth-mainnet.alchemyapi.io/v2/2FLlm9t-xOm0CbGx-ORr81li1yD_cKP6',
    );
    const [filterId, setFilterId] = useState<string | null>(null);
    const [timer, setTimer] = useState<any>();
    const [data, setData] = useState<Map<string, any>>(new Map());

    // don't know how to make it
    const updateTxInfos = (data: Array<string>) => {
        for (let item of data) {
            let itemCopy = item;
            getTxInfo(client!, serviceUrl, item)
                .then((x) => {
                    console.log(x);
                    setData((prev) => {
                        const result = new Map(prev);
                        if (x) {
                            result.set(itemCopy, x);
                        } else {
                            result.delete(itemCopy);
                        }
                        return result;
                    });
                })
                .catch((err) => {
                    console.log("couldn't get tx data", err);
                });
        }
    };

    const updateData = async (filterId) => {
        if (!filterId || !client) {
            return;
        }

        const data = await getFilterChanges(client, serviceUrl, filterId);
        console.log(data);
        setData((prev) => {
            const res = new Map(prev);
            for (let item of data) {
                res.set(item, 'nothing yet');
            }
            return res;
        });
    };

    useEffect(() => {
        const fn = async () => {
            try {
                const client = await createClient(relayNode);
                setClient(client);
            } catch (err) {
                console.log('Client initialization failed', err);
            }
        };
        fn();
    }, []);

    const start = async () => {
        if (!client) {
            return;
        }

        const filterId = await createFilter(client, serviceUrl);
        setFilterId(filterId);
        const timer = setInterval(updateData, intervalMs, filterId);
        setTimer(timer);
    };

    const stop = async () => {
        if (!filterId || !client) {
            return;
        }

        clearInterval(timer);
        setTimer(null);

        const res = await removeFilter(client, serviceUrl, filterId);
        console.log(res);
        setFilterId(null);
    };

    return (
        <>
            <div className="header-wrapper">
                <div className="header">
                    <div className="header-item"></div>

                    <div className="header-item">
                        Connection status: {client ? <span className="accent">connected</span> : 'disconnected'}
                    </div>
                </div>
            </div>
            <div className="content">
                <div>
                    node:
                    <input onChange={(e) => setServiceUrl(e.target.value)} type="text" value={serviceUrl} />
                </div>
                <div>
                    <button onClick={start}>start</button>
                </div>
                <div>
                    <button onClick={stop}>stop</button>
                </div>
                <div>count: {Array.from(data).length}</div>
                <div>
                    {Array.from(data).map((x) => {
                        const [hash, data] = x;
                        return (
                            <div key={hash}>
                                {hash} {data}
                            </div>
                        );
                    })}
                </div>
            </div>
        </>
    );
};

export default App;
