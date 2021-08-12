import React from 'react'

class TodoFooter extends React.Component {

    styles = {
        'title': {
            marginRight: 10,
            fontSize: 20
        },

        'top': {
            marginTop: 20
        }
    }

    handleAll = (e) => {
        let all = this.refs.all.value
        this.props.SubmitChooseValue(all)
    }

    handleActive = (e) => {
        let active = this.refs.active.value
        this.props.SubmitChooseValue(active)
    }

    handleComplete = (e) => {
        let complete = this.refs.complete.value
        this.props.SubmitChooseValue(complete)
    }

    render() {
        return (
            <div>
                <h2 style={this.styles.top}>show</h2>
                <button type='submit' style={this.styles.top} class='ui blue button' value='1' ref='all' onClick={this.handleAll}>全部</button>
                <button type='submit' style={this.styles.top} class='ui blue button' value='2' ref='active' onClick={this.handleActive}>还未完成</button>
                <button type='submit' style={this.styles.top} class='ui blue button' value='3' ref='complete' onClick={this.handleComplete}>已完成</button>
            </div>
        )
    }
}

export default TodoFooter
