const { makeHandler } = require('./utils')
const { getStargazersForRepo } = require('../data')
const { Repo } = require('../entities')

const inputSchema = {
  type: 'object',
  properties: {
    pathParameters: {
      type: 'object',
      properties: {
        ownerName: { type: 'string' },
        repoName: { type: 'string' }
      },
      required: [ 'ownerName', 'repoName' ]
    }
  },
  required: [ 'pathParameters' ]
}

const handler = async event => {
  const repo_obj = new Repo({
    ownerName: event.pathParameters.ownerName,
    repoName: event.pathParameters.repoName
  })
  const { repo, stargazers, error } = await getStargazersForRepo({ repo: repo_obj })
  const statusCode = error ? 500 : 200
  const body = error ? JSON.stringify({ error }) : JSON.stringify({ repo, stargazers })
  return {
    statusCode,
    body
  }
}

module.exports.handler = makeHandler({ handler, inputSchema })