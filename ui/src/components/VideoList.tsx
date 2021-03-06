import React from "react";
import axios from "axios";

interface Props {
	timerId?: number,
}
interface State {
	value: number,
}

export class VideoList extends React.Component<Props, State> {
	constructor(props: Props) {
		super(props);
		this.state = {
			value: 0,
		};
	}

	componentDidMount() {
		setInterval(()=> this.tick(), 1000);
	}
	componentWillUnmount() {
		clearInterval(this.props.timerId);
	}

	tick(){
		axios
			.get("/api/status")
			.then((results) => {
				const data = results.data;
				console.log(data);
			})
			.catch(() => {
				console.log("get status failed");
			})
		this.setState({
			value: this.state.value + 1,
		});
	}

	render(){
		return <h2>{this.state.value}</h2>;
	}
}
