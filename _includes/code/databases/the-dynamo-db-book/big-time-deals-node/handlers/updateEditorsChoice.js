const { makeHandler } = require('./utils')
const { updateEditorsChoice } = require('../data')
const { EditorsChoice} = require('../entities')

const inputSchema = {
  type: 'object',
  properties: {
    body: {
      type: 'object',
      properties: {
        featuredDeals: {
          type: 'array',
          items: {
            type: 'object',
            properties: {
              dealId: { type: 'string' },
              title: { type: 'string' },
              link: { type: 'string' },
              price: { type: 'number' },
              category: { type: 'string' },
              brand: { type: 'string' }
            },
            required: ['dealId', 'title', 'link', 'price', 'category', 'brand']
          },
          minItems: 1
        }
      },
      required: [ 'featuredDeals' ]
    }
  },
  required: [ 'body' ]
}

const handler = async event => {
  const page = new EditorsChoice({
    featuredDeals: event.body.featuredDeals,
  })
  const { error } = await updateEditorsChoice({ page })
  const statusCode = error ? 500 : 200
  const body = error ? JSON.stringify({ error }) : JSON.stringify({ ...page })
  return {
    statusCode,
    body
  }
}

module.exports.handler = makeHandler({ handler, inputSchema })