import { HoudiniClient } from '$houdini';

export default new HoudiniClient({
    url: 'https://api.studio.thegraph.com/query/25077/pwn/version/latest'

    // uncomment this to configure the network call (for things like authentication)
    // for more information, please visit here: https://www.houdinigraphql.com/guides/authentication
    // fetchParams({ session }) {
    //     return {
    //         headers: {
    //             Authentication: `Bearer ${session.token}`,
    //         }
    //     }
    // }
})
