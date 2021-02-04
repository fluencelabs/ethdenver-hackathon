import { createClient, FluenceClient, subscribeToEvent } from '@fluencelabs/fluence';
import React, { useEffect, useState } from 'react';
import {
    createFilter,
    getFilterChanges,
    getFilterChangesWithoutNulls,
    getTxInfo,
    relayNode,
    removeFilter,
    TxInfo,
} from 'src/fluence';

import './App.scss';

const intervalMs = 4000;

const App = () => {
    const [client, setClient] = useState<FluenceClient | null>(null);
    const [serviceUrl, setServiceUrl] = useState(
        'https://eth-mainnet.alchemyapi.io/v2/2FLlm9t-xOm0CbGx-ORr81li1yD_cKP6',
    );
    const [filterId, setFilterId] = useState<string | null>(null);
    const [timer, setTimer] = useState<any>();
    const [data, setData] = useState<TxInfo[]>([]);

    const updateData = async (filterId) => {
        if (!filterId || !client) {
            return;
        }

        try {
            const data = await getFilterChangesWithoutNulls(client, serviceUrl, filterId, '50');
            console.log(data);
            setData((prev) => {
                return [...data, ...prev];
            });
        } catch (err) {
            console.log('updateData failed', err);
        }
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

        try {
            const filterId = await createFilter(client, serviceUrl);
            setFilterId(filterId);
            const timer = setInterval(updateData, intervalMs, filterId);
            setTimer(timer);
        } catch (err) {
            console.log('createFilter failed', err);
        }
    };

    const stop = async () => {
        if (!filterId || !client) {
            return;
        }

        try {
            clearInterval(timer);
            setTimer(null);

            const res = await removeFilter(client, serviceUrl, filterId);
            console.log(res);
            setFilterId(null);
            setData([]);
        } catch (err) {
            console.log('stop failed', err);
        }
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
                    <input
                        className="text-input"
                        onChange={(e) => setServiceUrl(e.target.value)}
                        type="text"
                        value={serviceUrl}
                    />
                </div>
                <div className="buttons">
                    <button className="button" onClick={start}>
                        start
                    </button>

                    <button className="button" onClick={stop}>
                        stop
                    </button>
                </div>

                <div className="table-wrapper">
                    <table className="table">
                        <tr>
                            <th>from</th>
                            <th>to</th>
                            <th>gas</th>
                            <th>gas price</th>
                            <th>hash</th>
                        </tr>
                        {data.map((x) => (
                            <tr key={x.hash}>
                                <td className="td1">{x.from}</td>
                                <td className="td2">{x.to}</td>
                                <td className="td3">{x.gas}</td>
                                <td className="td4">{x.gasPrice}</td>
                                <td className="td5">{x.hash}</td>
                            </tr>
                        ))}
                    </table>
                </div>
            </div>
        </>
    );
};

export default App;
