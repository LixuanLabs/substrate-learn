import React, { useEffect, useState } from 'react'
import { Form, Grid } from 'semantic-ui-react'

import { useSubstrate } from './substrate-lib'
import { TxButton } from './substrate-lib/components'

import KittyCards from './KittyCards'

export default function Kitties (props) {
  const { api, keyring } = useSubstrate()
  const { accountPair } = props

  const [kitties, setKitties] = useState([])
  const [status, setStatus] = useState('')

  const getAddress = (value) => {
    // accountId 换取 address
    return keyring.getPairs().find(pair => {
      if (pair.addressRaw.join('') === value.join('')) {
        return true
      }
      return false
    })
    // const addresses = keyring.getPairs().map(i => i.ad);
  }

  const fetchKitties = () => {
    // TODO: 在这里调用 `api.query.kittiesModule.*` 函数去取得猫咪的信息。
    // 你需要取得：
    //   - 共有多少只猫咪
    //   - 每只猫咪的主人是谁
    //   - 每只猫咪的 DNA 是什么，用来组合出它的形态
    let unsub
    const getKittiesCount = async () => {
      unsub = await api.query.kittiesModule.kittiesCount(async ({ value }) => {
        const count = value.words[0]
        let index = 1
        const address = []
        while (index <= count) {
          address.push(index)
          index++
        }
        await api.query.kittiesModule.owner.multi(address, async (ownerData) => {
          const ownerValues = ownerData.filter(i => !i.isEmpty).map(i => getAddress(i.value).address)
          await api.query.kittiesModule.kitties.multi(address, (kittiesData) => {
            const kittiesValues = kittiesData.filter(i => !i.isEmpty).map(i => i.value)
            populateKitties(ownerValues, kittiesValues)
          })
        })
      })
    }
    getKittiesCount()

    return () => {
      unsub && unsub()
    }
  }

  const populateKitties = (ownerValues, kittiesValues) => {
    // TODO: 在这里添加额外的逻辑。你需要组成这样的数组结构：
    //  ```javascript
    //  const kitties = [{
    //    id: 0,
    //    dna: ...,
    //    owner: ...
    //  }, { id: ..., dna: ..., owner: ... }]
    //  ```
    // 这个 kitties 会传入 <KittyCards/> 然后对每只猫咪进行处理
    const kitties = []
    for (let index = 0; index < ownerValues.length; index++) {
      kitties.push({
        id: index,
        owner: ownerValues[index],
        dna: kittiesValues[index]
      })
    }

    setKitties(kitties)
  }

  const createKitties = async () => {
    await api.tx.kittiesModule.create()
  }

  useEffect(fetchKitties, [api, keyring])
  // useEffect(populateKitties, [])

  return <Grid.Column width={16}>
    <h1>小毛孩</h1>
    <KittyCards kitties={kitties} accountPair={accountPair} setStatus={setStatus}/>
    <Form style={{ margin: '1em 0' }}>
      <Form.Field style={{ textAlign: 'center' }}>
        <TxButton
          accountPair={accountPair} label='创建小毛孩' type='SIGNED-TX' setStatus={setStatus}
          attrs={{
            palletRpc: 'kittiesModule',
            callable: 'create',
            inputParams: [],
            paramFields: []
          }}
          onClick={createKitties}
        />
      </Form.Field>
    </Form>
    <div style={{ overflowWrap: 'break-word' }}>{status}</div>
  </Grid.Column>
}
